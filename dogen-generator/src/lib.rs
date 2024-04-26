mod map;
pub mod placename;
pub mod standard_map;
pub mod types;

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rayon::prelude::*;
    use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

    use crate::standard_map::StandardMap;

    #[test]
    fn test_standard_map() {
        let seed: u32 = rand::thread_rng().gen();
        println!("seed: {}", seed);

        let x_expand_prop = 1.4;

        let standard = &StandardMap::new(seed, x_expand_prop).unwrap();
        let img_width = (1000.0 * x_expand_prop) as u32;
        let img_height = 1000;

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

        let img_x_of = |x: f64| -> f64 {
            (x - standard.bound_min().x) / (standard.bound_max().x - standard.bound_min().x)
                * img_width as f64
        };

        let img_y_of = |y: f64| -> f64 {
            (y - standard.bound_min().y) / (standard.bound_max().y - standard.bound_min().y)
                * img_height as f64
        };

        let (bound_min, bound_max) = (standard.bound_min(), standard.bound_max());

        let pixels: Vec<_> = (0..img_width)
            .into_par_iter()
            .flat_map(|imgx| {
                (0..img_height).into_par_iter().filter_map(move |imgy| {
                    let x = bound_min.x
                        + (bound_max.x - bound_min.x) * ((imgx as f64 + 0.5) / img_width as f64);
                    let y = bound_min.y
                        + (bound_max.y - bound_min.y) * ((imgy as f64 + 0.5) / img_height as f64);
                    let elevation = standard.get_elevation(x, y);
                    if let Some(elevation) = elevation {
                        let color = get_color(elevation);
                        Some((imgx, imgy, color))
                    } else {
                        None
                    }
                })
            })
            .collect();

        let mut pixmap = Pixmap::new(img_width, img_height).unwrap();
        let mut paint = Paint::default();

        for (imgx, imgy, color) in pixels {
            paint.set_color_rgba8(color[0], color[1], color[2], 255);
            pixmap.fill_rect(
                Rect::from_xywh(imgx as f32, imgy as f32, 1.0, 1.0).unwrap(),
                &paint,
                Transform::identity(),
                None,
            );
        }

        standard.network_paths().iter().for_each(|path| {
            let (inode, jnode) = (path.node1(), path.node2());
            paint.set_color_rgba8(100, 100, 100, 255);

            let width = if jnode.stage().max(inode.stage()) == 0 {
                1.5
            } else {
                0.5
            };

            let stroke = Stroke {
                width,
                ..Default::default()
            };

            let path = {
                let mut path = PathBuilder::new();
                path.move_to(
                    img_x_of(inode.site().x) as f32,
                    img_y_of(inode.site().y) as f32,
                );
                path.line_to(
                    img_x_of(jnode.site().x) as f32,
                    img_y_of(jnode.site().y) as f32,
                );
                path.finish().unwrap()
            };

            paint.set_color_rgba8(0, 0, 0, 80);
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        });

        paint.set_color_rgba8(255, 0, 0, 255);
        // draw origin
        let origin = &standard.get_origin_site();
        pixmap.fill_rect(
            Rect::from_xywh(
                img_x_of(origin.x) as f32 - 2.0,
                img_y_of(origin.y) as f32 - 2.0,
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
