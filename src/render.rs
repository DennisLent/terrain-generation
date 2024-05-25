use crate::generation::Map;
use three_d::*;
use rayon::prelude::*;
use std::sync::Mutex;

// struct to contain the main information about a rectangle to be drawn
struct RectangleProperties {
    x_center: f32,
    y_center: f32,
    rect_color: Srgba,
}

// function to create a 2D top view of the terrain
// taken from https://github.com/asny/three-d/blob/master/examples/shapes2d/src/main.rs
pub fn render2d(map: Map) {

    let window_size: u32 = 800;

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
    let rect_properties: Vec<RectangleProperties> = map.land_map.par_iter().enumerate().flat_map_iter(|(i, row)| {
        row.iter().enumerate().map(move |(j, &value)| {
            let rect_color = if value == 0 {
                Srgba::BLUE
            } else {
                Srgba::GREEN
            };

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
                    vec2(props.x_center, props.y_center) * scale_factor,
                    degrees(0.0),
                    rect_size * scale_factor,
                    rect_size * scale_factor,
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


