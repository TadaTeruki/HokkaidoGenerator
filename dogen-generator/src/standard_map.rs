use fastlem::models::surface::sites::Site2D;
use rand::{rngs::StdRng, Rng, SeedableRng};
use street_engine::{
    core::{
        geometry::{angle::Angle, site::Site},
        Stage,
    },
    transport::{
        node::TransportNode,
        rules::{BranchRules, PathDirectionRules, TransportRules},
    },
};
use wasm_bindgen::prelude::*;

use crate::{
    map::{
        generator::{MapConfig, MapGenerator},
        terrain::TerrainConfig,
        Map,
    },
    placename::{NameConfig, NameGenerator},
};

#[wasm_bindgen]
pub fn create_standard_map(seed: u32, x_expand_prop: f64) -> Option<StandardMap> {
    StandardMap::new(seed, x_expand_prop)
}

#[wasm_bindgen]
#[derive(Debug, Clone)]

pub struct Name {
    name: String,
    reading: String,
}

#[wasm_bindgen]
impl Name {
    fn from_tuple(tuple: (String, String)) -> Self {
        Self {
            name: tuple.0,
            reading: tuple.1,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn reading(&self) -> String {
        self.reading.clone()
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct NameSet {
    city_name: Name,
    county_name: Name,
    subprefecture_name: Name,
    subprefecture_postfix: Name,
    government: Name,
}

#[wasm_bindgen]
impl NameSet {
    pub fn city_name(&self) -> Name {
        self.city_name.clone()
    }

    pub fn county_name(&self) -> Name {
        self.county_name.clone()
    }

    pub fn subprefecture_name(&self) -> Name {
        self.subprefecture_name.clone()
    }

    pub fn subprefecture_postfix(&self) -> Name {
        self.subprefecture_postfix.clone()
    }

    pub fn government(&self) -> Name {
        self.government.clone()
    }
}

#[wasm_bindgen]
pub struct MapSite {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct NetworkNode {
    transport_node: TransportNode,
}

#[wasm_bindgen]
impl NetworkNode {
    pub fn site(&self) -> MapSite {
        MapSite {
            x: self.transport_node.site.x,
            y: self.transport_node.site.y,
        }
    }
    pub fn stage(&self) -> usize {
        self.transport_node.stage.as_num()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct NetworkPath {
    node1: NetworkNode,
    node2: NetworkNode,
}

impl NetworkPath {
    pub fn node1(&self) -> NetworkNode {
        self.node1.clone()
    }

    pub fn node2(&self) -> NetworkNode {
        self.node2.clone()
    }
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

    pub fn new(seed: u32, x_expand_prop: f64) -> Option<StandardMap> {
        let mut rnd = StdRng::seed_from_u64(seed as u64);
        let land_ratio = rnd.gen_range(0.5..1.0);
        let city_size_prop_min = 0.01;
        let city_size_prop_max = 0.12;
        let city_size_prop = (city_size_prop_min
            + (city_size_prop_max - city_size_prop_min) * rnd.gen::<f64>().powi(4))
            * land_ratio;

        println!("city_size_prop: {}", city_size_prop);
        let mut namegen =
            NameGenerator::new(include_str!("../dataset/placenames.csv"), seed as usize);
        let city_name = Name::from_tuple(namegen.generate(NameConfig {
            target_name_length: 3.1 - city_size_prop * 20.0,
            cmp_samples: 5,
        })?);

        let map_config = MapConfig {
            sea_level: 1e-1,
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
            fault_scale: 0.1,
            erodibility_distribution_power: 4.0,
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
        let county_name_is_city_name = rnd.gen_bool(0.5) && (government.name != "村");
        let county_name = if county_name_is_city_name {
            city_name.clone()
        } else {
            Name::from_tuple(namegen.generate(NameConfig {
                target_name_length: 2.1,
                cmp_samples: 5,
            })?)
        };
        let subprefecture_name_is_city_name = (rnd.gen_bool(0.2) && government.name == "市")
            || (rnd.gen_bool(0.1) && government.name == "町");
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
            nameset: NameSet {
                city_name,
                county_name,
                subprefecture_name,
                subprefecture_postfix,
                government,
            },
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
                            Some(NetworkPath {
                                node1: NetworkNode {
                                    transport_node: inode,
                                },
                                node2: NetworkNode {
                                    transport_node: jnode,
                                },
                            })
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
        let seaside_prop = 1.0 - (elevation / 12.0).min(1.0).max(0.0);

        if stage.as_num() > 0 {
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
                    max_radian: std::f64::consts::PI
                        / (1.0 + 1450.0 * seaside_prop.powf(1.75) + 1000.0 * population_density),
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
                    branch_density: 0.05 + population_density * 0.95,
                    staging_probability: 0.99 - population_density * 0.2,
                },
                path_direction_rules: PathDirectionRules {
                    max_radian: std::f64::consts::PI
                        / (50.0 + 100.0 * seaside_prop + 10000.0 * population_density),
                    comparison_step: 5,
                },
            })
        }
    }
}
