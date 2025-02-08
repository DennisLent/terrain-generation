use bevy::prelude::*;
use pyri_tooltip::prelude::*;
use crate::world_generation::tile::WorldMap;

#[derive(Resource, Default)]
pub struct HoveredTile(Option<(usize, usize)>);

#[derive(Resource)]
pub struct CameraState {
    pub previous_transform: GlobalTransform,
    pub is_perspective: bool,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            previous_transform: GlobalTransform::default(),
            is_perspective: true,
        }
    }
}

pub fn update_hovered_tile(
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform, Option<&OrthographicProjection>, Option<&PerspectiveProjection>)>,
    mut hovered_tile: ResMut<HoveredTile>,
    mut world_map: ResMut<WorldMap>,
    mut camera_state: ResMut<CameraState>,
) {
    let window = windows.single();

    if let Some(cursor_pos) = window.cursor_position() {
        let (camera, global_transform, ortho, perspective) = q_camera.single();

        // Check if the transform or projection have changed
        if global_transform.translation() != camera_state.previous_transform.translation()
            || global_transform.rotation() != camera_state.previous_transform.rotation()
            || (ortho.is_some() && camera_state.is_perspective)
            || (perspective.is_some() && !camera_state.is_perspective)
        {
            // CameraState
            camera_state.previous_transform = *global_transform;
            camera_state.is_perspective = perspective.is_some();

            // Convert screen position to coordinates
            if let Ok(ray) = camera.viewport_to_world(global_transform, cursor_pos) {
                let world_pos = ray.origin;

                // Get tile indicies
                let tile_x = world_pos.x.floor() as isize;
                let tile_z = world_pos.z.floor() as isize;

                // Check if the calculated tiles are in bounds
                if tile_x >= 0
                    && tile_z >= 0
                    && (tile_x as usize) < world_map.get_size() as usize
                    && (tile_z as usize) < world_map.get_size() as usize
                {
                    hovered_tile.0 = Some((tile_x as usize, tile_z as usize));
                } else {
                    hovered_tile.0 = None;
                }
            } else {
                hovered_tile.0 = None;
            }
        }
    } else {
        hovered_tile.0 = None;
    }
}

pub fn update_tooltip(
    mut tooltip_query: Query<&mut Tooltip>,
    hovered_tile: Res<HoveredTile>,
    world_map: Res<WorldMap>,
    asset_server: Res<AssetServer>
) {
    if let Some((x, z)) = hovered_tile.0 {
        if let Some(biome) = world_map.tiles[x][z].biome {
            for mut tooltip in tooltip_query.iter_mut() {
                tooltip.content = TooltipContent::Primary(RichText::from_section(format!("Tile: ({:?},{:?}) Biome: {:?}", x, z, biome), TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                }));
                println!("Hovering over tile ({}, {}) with biome {:?}", x, z, biome);
            }
        }
    }
}
