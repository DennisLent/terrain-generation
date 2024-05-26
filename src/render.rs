use crate::generation::Map;
use three_d::*;
use rayon::prelude::*;
use std::sync::Mutex;

// colors for the different terrains
// ChatGPT made this because I was lazy
static SUBTROP_DESERT: Srgba = Srgba::new(237, 201, 175, 255);   // Light Sandy Brown
static SAVANNAH: Srgba = Srgba::new(238, 220, 130, 255);         // Light Goldenrod Yellow
static RAINFOREST: Srgba = Srgba::new(34, 139, 34, 255);         // Forest Green
static TEMP_RAIN: Srgba = Srgba::new(0, 128, 0, 255);            // Green
static FOREST: Srgba = Srgba::new(34, 139, 34, 255);             // Forest Green
static WOOLAND: Srgba = Srgba::new(139, 69, 19, 255);            // Saddle Brown
static GRASSLAND: Srgba = Srgba::new(124, 252, 0, 255);          // Lawn Green
static TAIGA: Srgba = Srgba::new(0, 100, 0, 255);                // Dark Green
static TUNDRA: Srgba = Srgba::new(176, 224, 230, 255);           // Powder Blue

/// struct to contain the main information about a rectangle to be drawn
struct RectangleProperties {
    x_center: f32,
    y_center: f32,
    rect_color: Srgba,
}

/// function to assign colors based on the type of terrain
fn assign_color(terrain_type: i32, temperature: f32, rainfall: f32) -> Srgba{
    // water for now just keep it blue
    if terrain_type == 0{
        Srgba::BLUE
    }
    // land tile so need to check temperature and rainfall
    else{
        // Determine the biome based on temperature and rainfall
        if temperature > 15.0 {
            if rainfall < 100.0 {
                SUBTROP_DESERT
            } else if rainfall < 250.0 {
                SAVANNAH
            } else {
                RAINFOREST
            }
        } else if temperature > 5.0 {
            if rainfall < 50.0 {
                GRASSLAND
            } else if rainfall < 120.0 {
                WOOLAND
            } 
            else if rainfall < 250.0 {
                FOREST
            }else {
                TEMP_RAIN
            }
        } else if temperature > 0.0 {
            if rainfall < 50.0 {
                TUNDRA
            } else {
                TAIGA
            }
        } else {
            TUNDRA
        }
    }
}

/// function to create a 2D top view of the terrain
/// taken from https://github.com/asny/three-d/blob/master/examples/shapes2d/src/main.rs
pub fn render2d(map: Map) {

    let window_size: u32 = 1000;

    // create window and context
    let window = Window::new(WindowSettings {
        title: "2D render".to_string(),
        max_size: Some((window_size, window_size)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();
    let scale_factor = window.device_pixel_ratio();
    let (window_width, window_height) = window.size();

    let mut shape_vector: Mutex<Vec<Gm<Rectangle, ColorMaterial>>> = Mutex::new(Vec::new());
    let tile_number = map.land_map.len() as f32;
    let rect_size = window_size as f32/tile_number;


    // Get all rectangle properties and store them in structs
    // This is done in parallel with rayon to reduce computation time
    let rain = &map.rainfall_map;
    let temp = &map.temperature_map;
    let rect_properties: Vec<RectangleProperties> = map.land_map.par_iter().enumerate().flat_map_iter(|(i, row)| {
        row.iter().enumerate().map(move |(j, &terrain)| {
            let temperature = temp[i][j];
            let rainfall = rain[i][j];

            let rect_color = assign_color(terrain, temperature, rainfall);

            // Determine center of the rectangle based on index in the land_map vector
            let x_center: f32 = (j as f32 + 0.5) * rect_size;
            let y_center: f32 = (i as f32 + 0.5) * rect_size;

            RectangleProperties {
                x_center,
                y_center,
                rect_color,
            }
        })
    }).collect();

        // Create rectangles
        // for now done sequentially, but maybe could be sped up 
        // TODO: Use an Arc or mutex for context
        let shapes: Vec<Gm<Rectangle, ColorMaterial>> = rect_properties.into_iter().map(|props| {
            Gm::new(
                Rectangle::new(
                    &context,
                    vec2(props.x_center, props.y_center),
                    degrees(0.0),
                    rect_size,
                    rect_size,
                ),
                ColorMaterial {
                    color: props.rect_color,
                    ..Default::default()
                },
            )
            
        }).collect();

    window.render_loop(move |frame_input| {
        for event in frame_input.events.iter() {
            if let Event::MousePress {
                button,
                position,
                modifiers,
                ..
            } = *event
            {
            }
        }
        // render shapevector on screen
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &Camera::new_2d(frame_input.viewport),
                &shapes,
                &[],
            );

        FrameOutput::default()
    });
}


