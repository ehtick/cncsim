use crate::utils::cncrouter;
use crate::utils::running_gcode;

use algorithms;
use draw::*;

use std::fs;

pub fn to_png<T>(
    image_size: (f64, f64),
    scale: f64,
    tools: Vec<cncrouter::Tool>,
    cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    non_cutting_box: ((f64, f64, f64), (f64, f64, f64)),
    safe_point: (f64, f64, f64),
    s: &mut T,
    file_name: &str,
) -> std::io::Result<(
    Vec<running_gcode::Warnings>,
    std::collections::HashMap<usize, f64>,
    f64,
)>
where
    T: Iterator<Item = char>,
{
    let mut canvas = Canvas::new(
        (image_size.0 as f64 * scale) as u32,
        (image_size.1 as f64 * scale) as u32,
    );

    let mut lowest_z = 0.0;
    let (warnings, time) = running_gcode::draw_path(
        tools,
        cutting_box,
        non_cutting_box,
        safe_point,
        s,
        |p1, p2, length, radius, color| {
            if p1.2 < lowest_z {
                lowest_z = p1.2;
            }
            if p2.2 < lowest_z {
                lowest_z = p2.2;
            }

            if p1.2 > 0.0 && p2.0 > 0.0 {
                return;
            }

            let line = LineBuilder::new((p1.0 * scale) as f32, (p1.1 * scale) as f32)
                .line_to((p2.0 * scale) as f32, (p2.1 * scale) as f32)
                .build();

            let circle = Shape::Circle {
                radius: (radius * scale) as u32,
            };

            let color = RGB {
                r: (255. * color.0) as u8,
                g: (255. * color.1) as u8,
                b: (255. * color.2) as u8,
            };
            canvas.display_list.add(
                Drawing::new()
                    .with_shape(line)
                    .with_xy(p1.0 as f32, p1.1 as f32)
                    .with_style(Style::stroked((2. * radius * scale) as u32, color)),
            );
            canvas.display_list.add(
                Drawing::new()
                    .with_shape(circle.clone())
                    .with_xy(p1.0 as f32, p1.1 as f32)
                    .with_style(Style::filled(color)),
            );
            canvas.display_list.add(
                Drawing::new()
                    .with_shape(circle)
                    .with_xy(p2.0 as f32, p2.1 as f32)
                    .with_style(Style::filled(color)),
            );
        },
    );

    // save the canvas as an svg
    render::save(&canvas, file_name, SvgRenderer::new()).expect("Failed to save");

    Ok((warnings, time, lowest_z))
}
