//! Create a custom material to draw basic lines in 3D

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

use crate::{ Collider, LEFT_WALL, RIGHT_WALL, TOP_WALL, BOTTOM_WALL, WALL_THICKNESS };

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    // sprite_bundle: SpriteBundle,
    mesh: MaterialMeshBundle<LineMaterial>,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

pub(crate) fn wall_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    // commands.spawn(WallBundle::new(&mut meshes, &mut materials, WallLocation::Right));
    // commands.spawn(WallBundle::new(&mut meshes, &mut materials, WallLocation::Left));
    // commands.spawn(WallBundle::new(&mut meshes, &mut materials, WallLocation::Bottom));
    // commands.spawn(WallBundle::new(&mut meshes, &mut materials, WallLocation::Top));

    for loc in &[WallLocation::Right, WallLocation::Left, WallLocation::Bottom, WallLocation::Top] {

        let (v1, v2) = loc.line();
        // Spawn a list of lines with start and end points for each lines
        commands.spawn(
            WallBundle {
                mesh: MaterialMeshBundle {
                    mesh: meshes.add(Mesh::from(LineStrip {
                        points: vec![
                            v1.extend(0.0),
                            v2.extend(0.0)
                        ],
                    })),
                    transform: Transform {
                        //                     // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                        //                     // This is used to determine the order of our sprites
                        translation: loc.position().extend(0.0),
                        //                     // The z-scale of 2D objects must always be 1.0,
                        //                     // or their ordering will be affected in surprising ways.
                        //                     // See https://github.com/bevyengine/bevy/issues/4149
                        scale: loc.size().extend(1.0),
                        ..default()
                    },
                    material: materials.add(LineMaterial {
                        color: Color::GREEN,
                    }),
                    ..default()
                },
                collider: Collider(loc.size())
            }
        );
    }

}

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "050ce6ac-080a-4d8c-b6b5-b5bab7560d8f"]
pub(crate) struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);

        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
}


impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(crate::LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(crate::RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., crate::BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., crate::TOP_WALL),
        }
    }

    fn line(&self) -> (Vec2, Vec2) {
        match self {
            WallLocation::Left => ( Vec2::new(LEFT_WALL, TOP_WALL), Vec2::new(LEFT_WALL, BOTTOM_WALL) ),
            WallLocation::Right => ( Vec2::new(RIGHT_WALL, TOP_WALL), Vec2::new(RIGHT_WALL, BOTTOM_WALL) ),
            WallLocation::Top => ( Vec2::new(LEFT_WALL, TOP_WALL), Vec2::new(RIGHT_WALL, TOP_WALL) ),
            WallLocation::Bottom => ( Vec2::new(LEFT_WALL, BOTTOM_WALL), Vec2::new(RIGHT_WALL, BOTTOM_WALL) ),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

// impl WallBundle {
//     // This "builder method" allows us to reuse logic across our wall entities,
//     // making our code easier to read and less prone to bugs when we change the logic
//     fn new(meshes: &mut Mesh, materials: &mut Material, location: WallLocation) -> WallBundle {
//         WallBundle {
//             sprite_bundle: SpriteBundle {
//                 transform: Transform {
//                     // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
//                     // This is used to determine the order of our sprites
//                     translation: location.position().extend(0.0),
//                     // The z-scale of 2D objects must always be 1.0,
//                     // or their ordering will be affected in surprising ways.
//                     // See https://github.com/bevyengine/bevy/issues/4149
//                     scale: location.size().extend(1.0),
//                     ..default()
//                 },
//                 sprite: Sprite {
//                     color: WALL_COLOR,
//                     ..default()
//                 },
//                 ..default()
//             },
//             mesh: MaterialMeshBundle {
//                 mesh: meshes.add(Mesh::from(LineList {
//                     lines: vec![
//                         (Vec3::ZERO, Vec3::new(1.0, 1.0, 0.0)),
//                         (Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
//                     ],
//                 })),
//                 transform: Transform::from_xyz(-1.5, 0.0, 0.0),
//                 material: materials.add(LineMaterial {
//                     color: Color::GREEN,
//                 }),
//                 ..default()
//             },
//             collider: Collider,
//         }
//     }
// }
