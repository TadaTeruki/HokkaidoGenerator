use fastlem::{core::traits::Model, models::surface::terrain::Terrain2D};
use naturalneighbor::Interpolator;
use street_engine::{
    core::geometry::site::Site,
    transport::traits::{RandomF64Provider, TransportRulesProvider},
};
use terrain_graph::edge_attributed_undirected::EdgeAttributedUndirectedGraph;

use super::terrain::{TerrainBuilder, TerrainConfig};

pub struct MapConfig {
    sea_level: f64,
    max_slope_livable: f64,
}

pub struct MapProvider<T>
where
    T: TransportRulesProvider,
{
    terrain: Terrain2D,
    population_densities: Vec<f64>,
    interpolator: Interpolator,
    transport_rules_provider: T,
    map_config: MapConfig,
}

impl<T> MapProvider<T>
where
    T: TransportRulesProvider,
{
    fn new(
        terrain_config: TerrainConfig,
        transport_rules_provider: T,
        map_config: MapConfig,
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
            transport_rules_provider,
            map_config,
        })
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
    //let max_slope_livable = std::f64::consts::PI / 3.0;
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
