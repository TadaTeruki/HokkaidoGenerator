use fastlem::{core::traits::Model, models::surface::terrain::Terrain2D};
use naturalneighbor::Interpolator;
use rand::SeedableRng;
use street_engine::{
    core::{
        geometry::{angle::Angle, site::Site},
        Stage,
    },
    transport::{
        builder::TransportBuilder,
        rules::TransportRules,
        traits::{RandomF64Provider, TransportRulesProvider},
    },
};
use terrain_graph::edge_attributed_undirected::EdgeAttributedUndirectedGraph;

use super::{
    terrain::{TerrainBuilder, TerrainConfig},
    Map,
};

#[derive(Debug, Clone)]
pub struct MapConfig {
    pub sea_level: f64,
    pub max_slope_livable: f64,
    pub origin_sample_num: usize,
    pub max_retries: usize,
    pub origin_min_evelation: f64,
    pub city_size_prop: f64,
}

pub struct MapGenerator<TF>
where
    TF: Fn(
        f64,   // elevation
        f64,   // population_density
        Site,  // site
        Angle, // angle
        f64,   // slope
        Stage, // stage
    ) -> Option<TransportRules>,
{
    terrain: Terrain2D,
    population_densities: Vec<f64>,
    interpolator: Interpolator,
    map_config: MapConfig,
    origin_site: Site,
    rules_fn: TF,
}

impl<TF> MapGenerator<TF>
where
    TF: Fn(f64, f64, Site, Angle, f64, Stage) -> Option<TransportRules>,
{
    pub fn new(
        terrain_config: TerrainConfig,
        map_config: MapConfig,
        rules_fn: TF,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Creating terrain...");
        let terrain_builder = TerrainBuilder::new(terrain_config.clone())?;
        let model = terrain_builder.get_model().clone();
        let terrain = terrain_builder.build()?;

        let mut rnd = RandomF64::new(rand::rngs::StdRng::seed_from_u64(0));

        let central_bound_min = terrain_config.central_bound_min();
        let central_bound_max = terrain_config.central_bound_max();

        let mut origin_site = None;
        for _ in 0..map_config.max_retries {
            origin_site = (0..map_config.origin_sample_num)
                .map(|_| {
                    let x = rnd.gen_f64() * (central_bound_max.x - central_bound_min.x)
                        + central_bound_min.x;
                    let y = rnd.gen_f64() * (central_bound_max.y - central_bound_min.y)
                        + central_bound_min.y;
                    Site { x, y }
                })
                .filter_map(|site| {
                    let elevation = terrain.get_elevation(&into_fastlem_site(site))?;
                    if elevation < map_config.origin_min_evelation {
                        return None;
                    }
                    Some((site, elevation))
                })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            if origin_site.is_some() {
                break;
            }
        }
        let origin_site = origin_site.ok_or("Failed to find origin site")?.0;

        let population_densities = calculate_population_density(
            &terrain,
            origin_site,
            &terrain_config,
            model.graph(),
            &map_config,
        );

        let interpolator = Interpolator::new(terrain.sites());

        Ok(Self {
            terrain,
            population_densities,
            interpolator,
            map_config,
            origin_site,
            rules_fn,
        })
    }

    pub fn build(self) -> Result<Map, Box<dyn std::error::Error>> {
        let mut rnd = RandomF64::new(rand::rngs::StdRng::seed_from_u64(0));

        let initial_angle = rnd.gen_f64() * std::f64::consts::PI * 2.0;

        println!("Creating transport network...");
        let network = TransportBuilder::new(&self)
            .add_origin(self.origin_site, initial_angle, None)
            .ok_or("Failed to add origin")?
            .iterate_as_possible(&mut rnd)
            .build();

        println!("Map generation completed.");

        let population = network
            .nodes_iter()
            .map(|(inode_id, inode)| {
                let p = network.neighbors_iter(inode_id).map(|neighbors_iter| {
                    neighbors_iter
                        .map(|(_, jnode)| {
                            let stage = jnode.stage.as_num().max(inode.stage.as_num());
                            if stage == 0 {
                                0.0
                            } else {
                                let site = Site {
                                    x: inode.site.x,
                                    y: inode.site.y,
                                };
                                self.interpolator
                                    .interpolate(
                                        &self.population_densities,
                                        naturalneighbor::Point {
                                            x: site.x,
                                            y: site.y,
                                        },
                                    )
                                    .unwrap_or(None)
                                    .unwrap_or(0.0)
                            }
                        })
                        .sum::<f64>()
                });
                p.unwrap_or(0.0)
            })
            .sum::<f64>()
            * 50.0;

        Ok(Map::new(
            self.terrain,
            self.interpolator,
            network,
            self.origin_site,
            initial_angle,
            population as usize,
        ))
    }
}

impl<TF> TransportRulesProvider for MapGenerator<TF>
where
    TF: Fn(f64, f64, Site, Angle, f64, Stage) -> Option<TransportRules>,
{
    fn get_rules(&self, site: &Site, angle: Angle, stage: Stage) -> Option<TransportRules> {
        let elevation = self.terrain.get_elevation(&into_fastlem_site(*site))?;
        let population_density = self
            .interpolator
            .interpolate(
                &self.population_densities,
                naturalneighbor::Point {
                    x: site.x,
                    y: site.y,
                },
            )
            .unwrap_or(None)?;
        if elevation < self.map_config.sea_level {
            return None;
        }
        /*
        let slope = self
            .terrain
            .get_slope(&into_fastlem_site(*site), angle.to_radians())?;
        */

        let slope_sample_distance = 1e-6;
        let site_to = site.extend(angle, slope_sample_distance);
        let slope = (elevation - self.terrain.get_elevation(&into_fastlem_site(site_to))?)
            / slope_sample_distance;

        (self.rules_fn)(elevation, population_density, *site, angle, slope, stage)
    }
}

pub struct RandomF64<R> {
    rng: R,
}

impl<R: rand::Rng> RandomF64<R> {
    fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: rand::Rng> RandomF64Provider for RandomF64<R> {
    fn gen_f64(&mut self) -> f64 {
        self.rng.gen()
    }
}

fn into_fastlem_site(site: Site) -> fastlem::models::surface::sites::Site2D {
    fastlem::models::surface::sites::Site2D {
        x: site.x,
        y: site.y,
    }
}

fn calculate_population_density(
    terrain: &Terrain2D,
    origin_site: Site,
    terrain_config: &TerrainConfig,
    graph: &EdgeAttributedUndirectedGraph<f64>,
    map_config: &MapConfig,
) -> Vec<f64> {
    let slopes = (0..terrain.sites().len())
        .map(|i| {
            let slopes = graph
                .neighbors_of(i)
                .iter()
                .map(|neighbor| {
                    let distance = neighbor.1;
                    let slope =
                        (terrain.elevations()[i] - terrain.elevations()[neighbor.0]) / distance;
                    slope.atan()
                })
                .collect::<Vec<_>>();
            slopes
        })
        .collect::<Vec<_>>();

    let densities = (0..terrain.sites().len())
        .map(|i| {
            let elevation = terrain.elevations()[i];
            if elevation < map_config.sea_level {
                return (i, 0.0);
            }
            let slope_sum = slopes[i].iter().fold(0.0, |acc, slope| acc + slope.abs());
            let slope_avg = slope_sum.abs() / slopes[i].len() as f64;
            let density = (1.0 - slope_avg / map_config.max_slope_livable)
                .max(0.0)
                .min(1.0);
            (i, density)
        })
        .map(|(i, density)| {
            // distance between origin and site
            let site = terrain.sites()[i];
            let distance = origin_site.distance(&Site {
                x: site.x,
                y: site.y,
            });
            let dprop = (1.0 - distance / terrain_config.bound_value())
                .min(1.0)
                .max(0.0)
                .powf(1.0 / map_config.city_size_prop);
            density * dprop
        })
        .collect::<Vec<_>>();

    densities
}
