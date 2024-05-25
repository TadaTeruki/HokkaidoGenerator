use std::f64::consts::PI;

use fastlem::models::surface::sites::Site2D;
use rand::{rngs::StdRng, Rng, SeedableRng};
use street_engine::{
    core::{
        geometry::{angle::Angle, site::Site},
        Stage,
    },
    transport::rules::{BranchRules, PathDirectionRules, TransportRules},
};
use wasm_bindgen::prelude::*;

use crate::{
    map::{
        generator::{MapConfig, MapGenerator},
        terrain::TerrainConfig,
        Map,
    },
    placename::{NameConfig, NameGenerator},
    types::{MapSite, Name, NameSet, NetworkNode, NetworkPath},
};

#[wasm_bindgen]
pub fn create_standard_map(seed: u32, x_expand_prop: f64, dataset: String) -> Option<StandardMap> {
    StandardMap::new(seed, x_expand_prop, dataset)
}

#[wasm_bindgen]
pub struct StandardMap {
    map: Map,
    bound_min: Site,
    bound_max: Site,
    nameset: NameSet,
}

#[wasm_bindgen]
impl StandardMap {
    fn create_map(
        terrain_config: TerrainConfig,
        map_config: MapConfig,
    ) -> Result<Map, Box<dyn std::error::Error>> {
        let map = MapGenerator::new(
            terrain_config.clone(),
            map_config.clone(),
            |elevation, population_density, site, angle, slope_rad, stage| {
                Self::rules_fn(
                    elevation,
                    population_density,
                    site,
                    angle,
                    slope_rad,
                    stage,
                    &map_config,
                )
            },
        )?
        .build()?;

        Ok(map)
    }

    pub fn new(seed: u32, x_expand_prop: f64, dataset: String) -> Option<StandardMap> {
        let mut rnd = StdRng::seed_from_u64(seed as u64);
        let land_ratio = rnd.gen_range(0.5..0.9);
        let erodibility_distribution_power = rnd.gen_range(3.0..5.0);
        let city_size_prop_min = 0.01;
        let city_size_prop_max = 0.12;
        let city_size_prop = (city_size_prop_min
            + (city_size_prop_max - city_size_prop_min) * rnd.gen::<f64>().powi(4))
            * land_ratio;

        println!("city_size_prop: {}", city_size_prop);
        let mut namegen = NameGenerator::new(dataset.as_str(), seed as usize);
        let city_name = Name::from_tuple(namegen.generate(NameConfig {
            target_name_length: 3.1 - city_size_prop * 20.0,
            cmp_samples: 5,
        })?);

        let map_config = MapConfig {
            sea_level: 1e-3,
            max_slope_livable: std::f64::consts::PI / 4.0,
            origin_sample_num: 10,
            max_retries: 500,
            origin_min_evelation: 2.0,
            city_size_prop,
        };

        let bound = 250.0;

        let terrain_config = TerrainConfig {
            x_bound: bound * x_expand_prop,
            y_bound: bound,
            seed,
            particle_num: 50000,
            fault_scale: 0.15,
            erodibility_distribution_power,
            land_ratio,
            convex_hull_is_always_outlet: false,
            global_max_slope: None,
        };

        let map = Self::create_map(terrain_config.clone(), map_config.clone()).ok()?;

        let gov_population = map.population + rnd.gen_range(0..map.population / 2);
        let government = if gov_population < 3000 {
            Name::from_tuple(("村".to_string(), "mura".to_string()))
        } else if gov_population < 20000 {
            Name::from_tuple(("町".to_string(), "cho".to_string()))
        } else {
            Name::from_tuple(("市".to_string(), "shi".to_string()))
        };
        let county_name_is_city_name = rnd.gen_bool(0.5) && (government.name() != "村");
        let county_name = if county_name_is_city_name {
            city_name.clone()
        } else {
            Name::from_tuple(namegen.generate(NameConfig {
                target_name_length: 2.1,
                cmp_samples: 5,
            })?)
        };
        let subprefecture_name_is_city_name = (rnd.gen_bool(0.2) && government.name() == "市")
            || (rnd.gen_bool(0.1) && government.name() == "町");
        let subprefecture_name_is_county_name = rnd.gen_bool(0.1);
        let subprefecture_name = if subprefecture_name_is_city_name {
            city_name.clone()
        } else if subprefecture_name_is_county_name {
            county_name.clone()
        } else {
            Name::from_tuple(namegen.generate(NameConfig {
                target_name_length: 2.1,
                cmp_samples: 5,
            })?)
        };

        let subprefecture_postfix = if rnd.gen_bool(0.8) {
            Name::from_tuple(("総合振興局".to_string(), "sogoshinkoukyoku".to_string()))
        } else {
            Name::from_tuple(("振興局".to_string(), "shinkoukyoku".to_string()))
        };

        let bound_min = terrain_config.bound_min();
        let bound_max = terrain_config.bound_max();

        Some(Self {
            map,
            bound_min: Site {
                x: bound_min.x,
                y: bound_min.y,
            },
            bound_max: Site {
                x: bound_max.x,
                y: bound_max.y,
            },
            nameset: NameSet::new(
                city_name,
                county_name,
                subprefecture_name,
                subprefecture_postfix,
                government,
            ),
        })
    }

    pub fn get_nameset(&self) -> NameSet {
        self.nameset.clone()
    }

    pub fn get_population(&self) -> usize {
        self.map.population
    }

    pub fn bound_min(&self) -> MapSite {
        MapSite {
            x: self.bound_min.x,
            y: self.bound_min.y,
        }
    }

    pub fn bound_max(&self) -> MapSite {
        MapSite {
            x: self.bound_max.x,
            y: self.bound_max.y,
        }
    }

    pub fn get_elevation(&self, x: f64, y: f64) -> Option<f64> {
        self.map.terrain.get_elevation(&Site2D { x, y })
    }

    pub fn network_paths(&self) -> Vec<NetworkPath> {
        self.map
            .network
            .nodes_iter()
            .flat_map(|(inode_id, &inode)| {
                let iter = self.map.network.neighbors_iter(inode_id);
                if let Some(iter) = iter {
                    iter.filter_map(|(jnode_id, &jnode)| {
                        if inode_id < jnode_id {
                            Some(NetworkPath::new(
                                NetworkNode::new(inode),
                                NetworkNode::new(jnode),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<NetworkPath>>()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }

    pub fn get_origin_site(&self) -> MapSite {
        MapSite {
            x: self.map.origin.x,
            y: self.map.origin.y,
        }
    }

    pub fn get_initial_angle(&self) -> f64 {
        self.map.initial_angle
    }

    fn rules_fn(
        elevation: f64,
        population_density: f64,
        _: Site,
        _: Angle,
        slope_rad: f64,
        stage: Stage,
        map_config: &MapConfig,
    ) -> Option<TransportRules> {
        if elevation < map_config.sea_level {
            return None;
        }

        let population_density = if stage.as_num() > 0 {
            population_density
        } else {
            population_density.max(0.001)
        };

        let slope_prop = slope_rad.abs() / (PI * 0.5);
        let path_priority = (1e-9 + population_density) * (-elevation - slope_prop);

        if stage.as_num() > 0 {
            Some(TransportRules {
                path_priority,
                elevation,
                population_density,
                path_normal_length: 0.5,
                path_extra_length_for_intersection: 0.3,
                branch_rules: BranchRules {
                    branch_density: 0.01
                        + (0.8 - slope_prop).max(0.0) * 0.005
                        + population_density * 0.985,
                    staging_probability: 0.0,
                },
                path_direction_rules: PathDirectionRules {
                    max_radian: std::f64::consts::PI
                        / (1.0
                            + 2500.0 * (0.6 - slope_prop).max(0.0)
                            + 5000.0 * population_density),
                    comparison_step: 7,
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
                    branch_density: 0.05 + population_density * 0.55,
                    staging_probability: 0.99 - (0.4 - slope_prop).max(0.0) * 0.1,
                },
                path_direction_rules: PathDirectionRules {
                    max_radian: std::f64::consts::PI / (40.0 + 800.0 * (0.4 - slope_prop).max(0.0)),
                    comparison_step: 5,
                },
            })
        }
    }
}
