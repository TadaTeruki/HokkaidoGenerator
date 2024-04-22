use fastlem::models::surface::terrain::Terrain2D;
use naturalneighbor::Interpolator;
use street_engine::{core::container::path_network::PathNetwork, transport::node::TransportNode};

pub struct Map {
    terrain: Terrain2D,
    population_densities: Vec<f64>,
    interpolator: Interpolator,
    network: PathNetwork<TransportNode>,
}

impl Map {
    pub(crate) fn new(
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
