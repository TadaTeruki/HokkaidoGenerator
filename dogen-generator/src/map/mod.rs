pub mod generator;
pub mod terrain;

use fastlem::models::surface::terrain::Terrain2D;
use naturalneighbor::Interpolator;
use street_engine::{core::container::path_network::PathNetwork, transport::node::TransportNode};

pub struct Map {
    pub terrain: Terrain2D,
    pub population_densities: Vec<f64>,
    pub interpolator: Interpolator,
    pub network: PathNetwork<TransportNode>,
}

impl Map {
    pub fn new(
        terrain: Terrain2D,
        population_densities: Vec<f64>,
        interpolator: Interpolator,
        network: PathNetwork<TransportNode>,
    ) -> Self {
        Self {
            terrain,
            population_densities,
            interpolator,
            network,
        }
    }
}
