use fastlem::models::surface::sites::Site2D;
use street_engine::{
    core::{
        geometry::{angle::Angle, site::Site},
        Stage,
    },
    transport::rules::{BranchRules, PathDirectionRules, TransportRules},
};

use crate::map::{
    generator::{MapConfig, MapGenerator},
    terrain::TerrainConfig,
    Map,
};

pub struct StandardMap {
    map: Map,
}

impl StandardMap {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let map_config = MapConfig {
            sea_level: 0.0,
            max_slope_livable: std::f64::consts::PI / 6.0,
        };

        let terrain_config = TerrainConfig {
            bound_min: Site2D {
                x: -100.0,
                y: -100.0,
            },
            bound_max: Site2D { x: 100.0, y: 100.0 },
            seed: 0,
            particle_num: 50000,
            fault_scale: 0.01,
            erodibility_distribution_power: 1.0,
            land_ratio: 0.3,
            convex_hull_is_always_outlet: false,
            global_max_slope: None,
        };
        let map = MapGenerator::new(terrain_config, map_config, Self::rules_fn)?
            .build()
            .unwrap();

        Ok(Self { map })
    }

    fn rules_fn(
        elevation: f64,
        population_density: f64,
        site: Site,
        angle: Angle,
        stage: Stage,
    ) -> Option<TransportRules> {
        let path_priority = (1e-9 + population_density) * (-elevation);

        if stage.as_num() > 0 {
            Some(TransportRules {
                path_priority,
                elevation,
                population_density,
                path_normal_length: 0.25,
                path_extra_length_for_intersection: 0.15,
                branch_rules: BranchRules {
                    branch_density: 0.01 + population_density * 0.99,
                    staging_probability: 0.0,
                },
                path_direction_rules: PathDirectionRules {
                    max_radian: std::f64::consts::PI / (5.0 + 1000.0 * population_density),
                    comparison_step: 3,
                },
            })
        } else {
            Some(TransportRules {
                path_priority: path_priority + 1e5,
                elevation,
                population_density,
                path_normal_length: 0.25,
                path_extra_length_for_intersection: 0.15,
                branch_rules: BranchRules {
                    branch_density: 0.2 + population_density * 0.8,
                    staging_probability: 0.97,
                },
                path_direction_rules: PathDirectionRules {
                    max_radian: std::f64::consts::PI / (10.0 + 100.0 * population_density),
                    comparison_step: 5,
                },
            })
        }
    }
}
