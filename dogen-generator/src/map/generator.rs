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

pub struct MapConfig {
    pub sea_level: f64,
    pub max_slope_livable: f64,
}

pub struct MapGenerator<TF>
where
    TF: Fn(
        f64,   // elevation
        f64,   // population_density
        Site,  // site
        Angle, // angle
        Stage, // stage
    ) -> Option<TransportRules>,
{
    terrain: Terrain2D,
    population_densities: Vec<f64>,
    interpolator: Interpolator,
    map_config: MapConfig,
    rules_fn: TF,
}

impl<TF> MapGenerator<TF>
where
    TF: Fn(f64, f64, Site, Angle, Stage) -> Option<TransportRules>,
{
    pub fn new(
        terrain_config: TerrainConfig,
        map_config: MapConfig,
        rules_fn: TF,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let terrain_builder = TerrainBuilder::new(terrain_config)?;
        let model = terrain_builder.get_model().clone();
        let terrain = terrain_builder.build()?;
        let population_densities =
            calculate_population_density(&terrain, model.graph(), &map_config);

        let interpolator = Interpolator::new(terrain.sites());

        Ok(Self {
            terrain,
            population_densities,
            interpolator,
            map_config,
            rules_fn,
        })
    }

    pub fn build(self) -> Result<Map, Box<dyn std::error::Error>> {
        let mut rnd = RandomF64::new(rand::rngs::StdRng::seed_from_u64(0));

        let network = TransportBuilder::new(&self)
            .add_origin(Site { x: 0.0, y: 0.0 }, 0.0, None)
            .ok_or("Failed to add origin")?
            .iterate_as_possible(&mut rnd)
            .build();

        Ok(Map::new(
            self.terrain,
            self.population_densities,
            self.interpolator,
            network,
        ))
    }
}

impl<TF> TransportRulesProvider for MapGenerator<TF>
where
    TF: Fn(f64, f64, Site, Angle, Stage) -> Option<TransportRules>,
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

        (self.rules_fn)(elevation, population_density, *site, angle, stage)
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
                return 0.0;
            }
            let slope_sum = slopes[i].iter().fold(0.0, |acc, slope| acc + slope.abs());
            let slope_avg = slope_sum.abs() / slopes[i].len() as f64;
            (1.0 - slope_avg / map_config.max_slope_livable)
                .max(0.0)
                .min(1.0)
        })
        .collect::<Vec<_>>();

    densities
}
