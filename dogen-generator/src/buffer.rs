use crate::standard_map::StandardMap;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ElevationBuffer {
    buffer: Vec<f64>,
    image_width: u32,
}

#[wasm_bindgen]
impl ElevationBuffer {
    pub fn from_terrain(
        standard: &StandardMap,
        image_width: u32,
        image_height: u32,
    ) -> ElevationBuffer {
        let (bound_min, bound_max) = (standard.bound_min(), standard.bound_max());

        let buffer = (0..image_height)
            .into_par_iter()
            .flat_map(|imagey| {
                (0..image_width).into_par_iter().map(move |imagex| {
                    let x = bound_min.x
                        + (bound_max.x - bound_min.x)
                            * ((imagex as f64 + 0.5) / image_width as f64);
                    let y = bound_min.y
                        + (bound_max.y - bound_min.y)
                            * ((imagey as f64 + 0.5) / image_height as f64);
                    let elevation = standard.get_elevation(x, y);
                    if let Some(elevation) = elevation {
                        elevation
                    } else {
                        0.0
                    }
                })
            })
            .collect();

        ElevationBuffer {
            buffer,
            image_width,
        }
    }

    pub fn get_elevation(&self, x: u32, y: u32) -> f64 {
        self.buffer[(y * self.image_width + x) as usize]
    }
}
