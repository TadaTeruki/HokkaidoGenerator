pub mod buffer;
mod map;
pub mod placename;
pub mod standard_map;
pub mod types;

#[cfg(test)]
mod tests {
    use rand::Rng;
    use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

    use crate::{buffer::ElevationBuffer, standard_map::StandardMap};

    #[test]
    fn test_standard_map() {
        let seed: u32 = rand::thread_rng().gen();
        println!("seed: {}", seed);

        let x_expand_prop = 1.4;

        let standard = &StandardMap::new(
            seed,
            x_expand_prop,
            include_str!("../../frontend/static/dataset/placenames.csv").into(),
        )
        .unwrap();
        let image_width = (1000.0 * x_expand_prop) as u32;
        let image_height = 1000;

        let address = {
            let subprefecture = format!(
                "{}{}",
                standard.get_nameset().subprefecture_name().name(),
                standard.get_nameset().subprefecture_postfix().name()
            );
            if standard.get_nameset().government().name() == "市" {
                format!(
                    "{} {}{}",
                    subprefecture,
                    standard.get_nameset().city_name().name(),
                    standard.get_nameset().government().name()
                )
            } else {
                format!(
                    "{} {}郡{}{}",
                    subprefecture,
                    standard.get_nameset().county_name().name(),
                    standard.get_nameset().city_name().name(),
                    standard.get_nameset().government().name()
                )
            }
        };

        println!(
            "{}市街 ({})",
            standard.get_nameset().city_name().name(),
            address
        );
        println!("人口 {}人", standard.get_population());

        let blend_color = |color_a: [u8; 3], color_b: [u8; 3], prop: f64| -> [u8; 3] {
            [
                (color_a[0] as f64 + (color_b[0] as f64 - color_a[0] as f64) * prop) as u8,
                (color_a[1] as f64 + (color_b[1] as f64 - color_a[1] as f64) * prop) as u8,
                (color_a[2] as f64 + (color_b[2] as f64 - color_a[2] as f64) * prop) as u8,
            ]
        };

        let get_color = |elevation: f64| -> [u8; 3] {
            let colormap: [([u8; 3], f64); 6] = [
                ([70, 150, 200], 0.0),
                ([70, 150, 200], 0.05),
                ([240, 240, 210], 0.125),
                ([190, 200, 120], 0.5),
                ([25, 100, 25], 25.0),
                ([15, 60, 15], 40.0),
            ];
            let color_index = {
                let mut i = 0;
                while i < colormap.len() {
                    if elevation < colormap[i].1 {
                        break;
                    }
                    i += 1;
                }
                i
            };

            let land_color = if color_index == 0 {
                colormap[0].0
            } else if color_index == colormap.len() {
                colormap[colormap.len() - 1].0
            } else {
                let color_a = colormap[color_index - 1];
                let color_b = colormap[color_index];

                let prop = (elevation - color_a.1) / (color_b.1 - color_a.1);
                blend_color(color_a.0, color_b.0, prop)
            };
            land_color
        };

        let elevation_buffer = &ElevationBuffer::from_terrain(standard, image_width, image_height);

        let pixels = (0..image_width).flat_map(|imagex| {
            (0..image_height).map(move |imagey| {
                let elevation = elevation_buffer.get_elevation(imagex, imagey);
                let color = get_color(elevation);
                (imagex, imagey, color)
            })
        });

        let image_x_of = |x: f64| -> f64 {
            (x - standard.bound_min().x) / (standard.bound_max().x - standard.bound_min().x)
                * image_width as f64
        };

        let image_y_of = |y: f64| -> f64 {
            (y - standard.bound_min().y) / (standard.bound_max().y - standard.bound_min().y)
                * image_height as f64
        };

        let mut pixmap = Pixmap::new(image_width, image_height).unwrap();
        let mut paint = Paint::default();

        for (imagex, imagey, color) in pixels {
            paint.set_color_rgba8(color[0], color[1], color[2], 255);
            pixmap.fill_rect(
                Rect::from_xywh(imagex as f32, imagey as f32, 1.0, 1.0).unwrap(),
                &paint,
                Transform::identity(),
                None,
            );
        }

        standard.network_paths().iter().for_each(|path| {
            let (inode, jnode) = (path.node1(), path.node2());

            let width = if path.stage() == 0 { 1.5 } else { 0.5 };

            let stroke = Stroke {
                width,
                ..Default::default()
            };

            let dpath = {
                let mut path = PathBuilder::new();
                path.move_to(
                    image_x_of(inode.site().x) as f32,
                    image_y_of(inode.site().y) as f32,
                );
                path.line_to(
                    image_x_of(jnode.site().x) as f32,
                    image_y_of(jnode.site().y) as f32,
                );
                path.finish().unwrap()
            };
            paint.set_color_rgba8(0, 0, 0, 255);
            pixmap.stroke_path(&dpath, &paint, &stroke, Transform::identity(), None);
        });

        paint.set_color_rgba8(255, 0, 0, 255);
        // draw origin
        let origin = &standard.get_origin_site();
        pixmap.fill_rect(
            Rect::from_xywh(
                image_x_of(origin.x) as f32 - 2.0,
                image_y_of(origin.y) as f32 - 2.0,
                4.0,
                4.0,
            )
            .unwrap(),
            &paint,
            Transform::identity(),
            None,
        );

        pixmap.save_png("test.png").unwrap();
    }
}
