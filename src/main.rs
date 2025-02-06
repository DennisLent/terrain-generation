mod generate_voxel_world;
use bevy::render::mesh::Mesh;
use bevy::{
    color::palettes::css::*,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use std::f32::consts::PI;
use generate_voxel_world::generate_chunk::generate_chunk;
use generate_voxel_world::meshing::mesh_chunk;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            // You need to add this plugin to enable wireframe rendering
            WireframePlugin,
            PanOrbitCameraPlugin,
        ))
        // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: false,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: WHITE.into(),
        })
        .add_systems(Startup, startup)
        .add_systems(Update, toggle_wireframe)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(0.0, 50.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    // Generate and spawn multiple chunks
    let chunk_size = 32; // Adjust this to match CHUNK_SIZE
    let world_size = 2; // Number of chunks in each direction

    for chunk_x in -world_size..world_size {
        for chunk_z in -world_size..world_size {
            let chunk_voxels = generate_chunk(chunk_x as i32, chunk_z as i32, 42);
            let chunk_mesh = mesh_chunk(&chunk_voxels);

            println!(
                "Spawning chunk at position: ({}, {}, {})",
                chunk_x as f32 * chunk_size as f32,
                0.0,
                chunk_z as f32 * chunk_size as f32
            );

            commands.spawn((
                Mesh3d(meshes.add(chunk_mesh)),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_xyz(
                    chunk_x as f32 * chunk_size as f32,
                    0.0,
                    chunk_z as f32 * chunk_size as f32,
                ),
                Terrain,
            ));
        }
    }

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));
}

#[derive(Component)]
struct Terrain;

/// Quick way to toggle wireframes
fn toggle_wireframe(
    mut commands: Commands,
    landscape_wireframes: Query<Entity, (With<Terrain>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for terrain in &landscapes {
            commands.entity(terrain).insert(Wireframe);
        }
        for terrain in &landscape_wireframes {
            commands.entity(terrain).remove::<Wireframe>();
        }
    }
}
