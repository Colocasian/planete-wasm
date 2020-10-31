/*
 * Copyright 2020 Rishvic Pushpakaran
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    consts::{FASTER, MULT},
    planet::Planet,
};

use wasm_bindgen::prelude::*;

/// Struct simulating a system of planets.
#[wasm_bindgen]
pub struct PlanetSystem {
    planets: Vec<Planet>,
}

#[wasm_bindgen]
impl PlanetSystem {
    /// Instantiates a PlanetSystem.
    pub fn new() -> PlanetSystem {
        PlanetSystem { planets: vec![] }
    }

    /// Method to add a planet to the system.
    pub fn add_planet(&mut self, m: f64, x: f64, y: f64, init_vx: f64, init_vy: f64) {
        self.planets.push(Planet::new(m, x, y, init_vx, init_vy));
    }

    /// Method to refresh the system of planets one frame
    pub fn refresh(&mut self) {
        for _ in 0..(FASTER * MULT) as u128 {
            for i in 1..self.planets.len() {
                let (x, y) = self.planets.split_at_mut(i);
                if let Some(planet_x) = x.last_mut() {
                    for planet_y in y.iter_mut() {
                        planet_x.add_accel(planet_y);
                        planet_y.add_accel(planet_x);
                    }
                }
            }

            for planet in self.planets.iter_mut() {
                planet.refresh_new();
            }
        }
    }

    /// Outputs every planet in a list of string.
    pub fn output(&self) -> String {
        let mut ans: String = String::new();
        for planet in self.planets.iter() {
            ans.push_str(format!("{},{}\n", planet.get_x(), planet.get_y(),).as_str())
        }
        ans
    }
}
