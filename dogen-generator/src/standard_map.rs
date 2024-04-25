use rand::{rngs::StdRng, Rng, SeedableRng};
use street_engine::{
    core::{
        geometry::{angle::Angle, site::Site},
        Stage,
    },
    transport::rules::{BranchRules, PathDirectionRules, TransportRules},
};

use crate::{
    map::{
        generator::{MapConfig, MapGenerator},
        terrain::TerrainConfig,
        Map,
    },
    placename::{NameConfig, NameGenerator},
};

pub struct StandardMap {
    map: Map,
    terrain_config: TerrainConfig,
    city_name: String,
    county_name: String,
    subprefecture_name: String,
    subprefecture_postfix: String,
    government: String,
}

impl StandardMap {
    pub fn create_map(
        terrain_config: TerrainConfig,
        map_config: MapConfig,
    ) -> Result<Map, Box<dyn std::error::Error>> {
        let map = MapGenerator::new(
            terrain_config.clone(),
            map_config.clone(),
            |elevation, population_density, site, angle, stage| {
                Self::rules_fn(
                    elevation,
                    population_density,
                    site,
                    angle,
                    stage,
                    map_config.sea_level,
                )
            },
        )?
        .build()?;

        Ok(map)
    }

    pub fn new(seed: u32, source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rnd = StdRng::seed_from_u64(seed as u64);
        let land_ratio = rnd.gen_range(0.5..1.0);
        let city_size_prop_min = 0.01;
        let city_size_prop_max = 0.12;
        let city_size_prop = (city_size_prop_min
            + (city_size_prop_max - city_size_prop_min) * rnd.gen::<f64>().powi(4))
            * land_ratio;

        println!("city_size_prop: {}", city_size_prop);
        let mut namegen = NameGenerator::new(source, seed as usize);
        let city_name = namegen
            .generate(NameConfig {
                target_name_length: 3.1 - city_size_prop * 20.0,
                cmp_samples: 5,
            })
            .ok_or("Failed to generate city name")?
            .0;

        let map_config = MapConfig {
            sea_level: 1e-1,
            max_slope_livable: std::f64::consts::PI / 3.0,
            origin_sample_num: 10,
            max_retries: 500,
            origin_min_evelation: 2.0,
            city_size_prop,
        };

        let terrain_config = TerrainConfig {
            bound: 250.0,
            seed,
            particle_num: 50000,
            fault_scale: 0.1,
            erodibility_distribution_power: 3.0,
            land_ratio,
            convex_hull_is_always_outlet: false,
            global_max_slope: None,
        };

        let map = Self::create_map(terrain_config.clone(), map_config.clone())?;

        let gov_population = map.population + rnd.gen_range(0..map.population / 2);
        let government = if gov_population < 3000 {
            "村"
        } else if gov_population < 20000 {
            "町"
        } else {
            "市"
        };
        let county_name_is_city_name = rnd.gen_bool(0.5) && (government != "村");
        let county_name = if county_name_is_city_name {
            city_name.clone()
        } else {
            namegen
                .generate(NameConfig {
                    target_name_length: 2.1,
                    cmp_samples: 5,
                })
                .ok_or("Failed to generate county name")?
                .0
        };
        let subprefecture_name_is_city_name =
            (rnd.gen_bool(0.2) && government == "市") || (rnd.gen_bool(0.1) && government == "町");
        let subprefecture_name_is_county_name = rnd.gen_bool(0.1);
        let subprefecture_name = if subprefecture_name_is_city_name {
            city_name.clone()
        } else if subprefecture_name_is_county_name {
            county_name.clone()
        } else {
            namegen
                .generate(NameConfig {
                    target_name_length: 2.1,
                    cmp_samples: 5,
                })
                .ok_or("Failed to generate subprefecture name")?
                .0
        };

        let subprefecture_postfix = if rnd.gen_bool(0.8) {
            "総合振興局"
        } else {
            "振興局"
        };

        Ok(Self {
            map,
            terrain_config,
            city_name,
            county_name,
            subprefecture_name: String::from(subprefecture_name),
            subprefecture_postfix: String::from(subprefecture_postfix),
            government: String::from(government),
        })
    }

    pub fn get_address(&self) -> String {
        let subprefecture = format!("{}{}", self.subprefecture_name, self.subprefecture_postfix);
        if self.government == "市" {
            format!("{} {}{}", subprefecture, self.city_name, self.government)
        } else {
            format!(
                "{} {}郡{}{}",
                subprefecture, self.county_name, self.city_name, self.government
            )
        }
    }

    fn rules_fn(
        elevation: f64,
        population_density: f64,
        _: Site,
        _: Angle,
        stage: Stage,
        sea_level: f64,
    ) -> Option<TransportRules> {
        if elevation < sea_level {
            return None;
        }

        let population_density = if stage.as_num() > 0 {
            population_density
        } else {
            population_density.max(0.001)
        };

        let path_priority = (1e-9 + population_density) * (-elevation);

        if stage.as_num() > 0 {
            if population_density == 0.0 {
                return None;
            }

            Some(TransportRules {
                path_priority,
                elevation,
                population_density,
                path_normal_length: 0.5,
                path_extra_length_for_intersection: 0.3,
                branch_rules: BranchRules {
                    branch_density: 0.01 + population_density * 0.99,
                    staging_probability: 0.0,
                },
                path_direction_rules: PathDirectionRules {
                    max_radian: std::f64::consts::PI / (1500.0 + 1000.0 * population_density),
                    comparison_step: 3,
                },
            })
        } else {
            Some(TransportRules {
                path_priority: path_priority + 1e5,
                elevation,
                population_density,
                path_normal_length: 0.5,
                path_extra_length_for_intersection: 0.3,
                branch_rules: BranchRules {
                    branch_density: 0.1 + population_density * 0.9,
                    staging_probability: 0.99 - population_density * 0.2,
                },
                path_direction_rules: PathDirectionRules {
                    max_radian: std::f64::consts::PI / (50.0 + 10000.0 * population_density),
                    comparison_step: 5,
                },
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use fastlem::models::surface::sites::Site2D;
    use rayon::prelude::*;
    use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

    use super::*;

    #[test]
    fn test_standard_map() {
        let seed: u32 = rand::thread_rng().gen();

        let standard = StandardMap::new(seed, include_str!("../dataset/placenames.csv")).unwrap();
        let img_width = 1000;
        let img_height = 1000;
        println!("{}市街 ({})", standard.city_name, standard.get_address());
        println!("人口 {}人", standard.map.population);

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
            (x - standard.terrain_config.bound_min().x)
                / (standard.terrain_config.bound_max().x - standard.terrain_config.bound_min().x)
                * img_width as f64
        };

        let img_y_of = |y: f64| -> f64 {
            (y - standard.terrain_config.bound_min().y)
                / (standard.terrain_config.bound_max().y - standard.terrain_config.bound_min().y)
                * img_height as f64
        };

        let (bound_min, bound_max) = (
            standard.terrain_config.bound_min(),
            standard.terrain_config.bound_max(),
        );

        let terrain = &standard.map.terrain;

        let pixels: Vec<_> = (0..img_width)
            .into_par_iter()
            .flat_map(|imgx| {
                (0..img_height).into_par_iter().filter_map(move |imgy| {
                    let x = bound_min.x
                        + (bound_max.x - bound_min.x) * ((imgx as f64 + 0.5) / img_width as f64);
                    let y = bound_min.y
                        + (bound_max.y - bound_min.y) * ((imgy as f64 + 0.5) / img_height as f64);
                    let site = Site2D { x, y };
                    let elevation = terrain.get_elevation(&site);
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

        let network = &standard.map.network;

        network.nodes_iter().for_each(|(inode_id, inode)| {
            // draw node
            network.neighbors_iter(inode_id).map(|neighbors_iter| {
                neighbors_iter.for_each(|(_, jnode)| {
                    paint.set_color_rgba8(100, 100, 100, 255);

                    let width = if jnode.stage.as_num().max(inode.stage.as_num()) == 0 {
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
                        path.move_to(img_x_of(inode.site.x) as f32, img_y_of(inode.site.y) as f32);
                        path.line_to(img_x_of(jnode.site.x) as f32, img_y_of(jnode.site.y) as f32);
                        path.finish().unwrap()
                    };

                    paint.set_color_rgba8(0, 0, 0, 80);
                    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
                })
            });
        });

        paint.set_color_rgba8(255, 0, 0, 255);
        // draw origin
        let origin = &standard.map.origin;
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
