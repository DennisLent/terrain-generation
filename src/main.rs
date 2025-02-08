mod world_generation;
mod utils;
use bevy::{
    color::palettes::css::*,
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::{RenderCreation, WgpuSettings}, RenderPlugin},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy::render::mesh::Mesh;
use std::f32::consts::PI;
use pyri_tooltip::prelude::*;
use world_generation::meshing::generate_terrain_mesh;
use utils::mouse::{update_hovered_tile, HoveredTile, update_tooltip, CameraState};

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
        // Add the tooltip plugin
        .add_plugins(TooltipPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, toggle_wireframe)
        .insert_resource(HoveredTile::default())
        .add_systems(Update, update_hovered_tile)
        .add_systems(Update, update_tooltip)
        .run();
}

fn startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Spawn camera
    commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(0.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    commands.insert_resource(CameraState::default());

    // how many 4096 chunks we will have
    let world_chunks: f32 = 4.0;
    let world_size: f32 = world_chunks*4096.0;


    let (mesh, world_map) = generate_terrain_mesh(world_size, 1000, 1);
    println!("WORLD GENERATED!");

    // Insert world_map as resource
    commands.insert_resource(world_map);

    // Spawn mesh
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Terrain,
    ));

    // Spawn lighting
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));

    // Spawn tooltip UI
    commands.spawn(Tooltip::cursor("Hover over a tile"));
    
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
