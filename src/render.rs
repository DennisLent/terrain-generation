use crate::map::Map;
use three_d::*;

pub fn render2d(map: Map) {
    let window = Window::new(WindowSettings {
        title: "2D Render".to_string(),
        max_size: Some((800, 600)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();
    let scale_factor = window.device_pixel_ratio();
    let (width, height) = window.size();

    let rect_size = 20.0;

    window.render_loop(move |frame_input| {

        let mut frame = FrameOutput::new(&context);

        for (i, row) in map.land_map.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value == 1 {
                    let center_value =
                        vec2((j as f32 * rect_size) as f32, (i as f32 * rect_size) as f32);
                    let rectangle = Gm::new(
                        Rectangle::new(
                            &context,
                            center_value * scale_factor,
                            degrees(0.0),
                            rect_size * scale_factor,
                            rect_size * scale_factor,
                        ),
                        ColorMaterial {
                            color: Srgba::GREEN,
                            ..Default::default()
                        },
                    );

                } else {
                    let center_value =
                        vec2((j as f32 * rect_size) as f32, (i as f32 * rect_size) as f32);
                    Gm::new(
                        Rectangle::new(
                            &context,
                            center_value * scale_factor,
                            degrees(0.0),
                            rect_size * scale_factor,
                            rect_size * scale_factor,
                        ),
                        ColorMaterial {
                            color: Srgba::BLUE,
                            ..Default::default()
                        },
                    );
                }

            }
        }

        FrameOutput::default()
    })
}


