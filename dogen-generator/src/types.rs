use street_engine::transport::node::TransportNode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]

pub struct Name {
    name: String,
    reading: String,
}

#[wasm_bindgen]
impl Name {
    pub(crate) fn from_tuple(tuple: (String, String)) -> Self {
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
    pub(crate) fn new(
        city_name: Name,
        county_name: Name,
        subprefecture_name: Name,
        subprefecture_postfix: Name,
        government: Name,
    ) -> Self {
        Self {
            city_name,
            county_name,
            subprefecture_name,
            subprefecture_postfix,
            government,
        }
    }

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
    pub(crate) fn new(transport_node: TransportNode) -> Self {
        Self { transport_node }
    }

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
    pub(crate) fn new(node1: NetworkNode, node2: NetworkNode) -> Self {
        Self { node1, node2 }
    }

    pub fn node1(&self) -> NetworkNode {
        self.node1.clone()
    }

    pub fn node2(&self) -> NetworkNode {
        self.node2.clone()
    }
}
