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
}

impl Map {
    pub fn new(
        terrain: Terrain2D,
        interpolator: Interpolator,
        network: PathNetwork<TransportNode>,
        origin: Site,
    ) -> Self {
        Self {
            terrain,
            interpolator,
            network,
            origin,
        }
    }
}
