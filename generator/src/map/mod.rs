pub mod generator;
pub mod terrain;

use fastlem::models::surface::terrain::Terrain2D;
use naturalneighbor::Interpolator;
use street_engine::{
    core::{container::path_network::PathNetwork, geometry::site::Site},
    transport::node::TransportNode,
};

pub struct Map {
    pub terrain: Terrain2D,
    pub interpolator: Interpolator,
    pub network: PathNetwork<TransportNode>,
    pub origin: Site,
    pub initial_angle: f64,
    pub population: usize,
}

impl Map {
    pub fn new(
        terrain: Terrain2D,
        interpolator: Interpolator,
        network: PathNetwork<TransportNode>,
        origin: Site,
        initial_angle: f64,
        population: usize,
    ) -> Self {
        Self {
            terrain,
            interpolator,
            network,
            origin,
            initial_angle,
            population,
        }
    }
}
