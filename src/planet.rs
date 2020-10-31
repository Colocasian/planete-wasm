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

use crate::consts::{G, REP};

/// Struct representing a planet.
#[derive(Copy, Clone)]
pub struct Planet {
    m: f64,
    x: f64,
    y: f64,
    x_p: f64,
    y_p: f64,
    net_acc: [f64; 2],
}

impl Planet {
    /// Instantiates a new Planet struct.
    pub fn new(m: f64, x: f64, y: f64, init_vx: f64, init_vy: f64) -> Planet {
        Planet {
            m,
            x,
            y,
            x_p: x - init_vx / REP,
            y_p: y - init_vy / REP,
            net_acc: [0., 0.],
        }
    }

    /// Method to find acceleration of planet w.r.t. another planet.
    fn acc(&self, b2: &Planet) -> [f64; 2] {
        let x_dir = b2.x - self.x;
        let y_dir = b2.y - self.y;
        let num = x_dir * x_dir + y_dir * y_dir;

        let a_mod = G * b2.m / num;
        [x_dir / num.sqrt() * a_mod, y_dir / num.sqrt() * a_mod]
    }

    /// Method to keep in check the net acceleration on the planet.
    pub fn add_accel(&mut self, b2: &Planet) {
        let acc_add = self.acc(b2);
        self.net_acc[0] += acc_add[0];
        self.net_acc[1] += acc_add[1];
    }

    /// Refreshes the location of the planet for the given net acceleration
    /// value stored.
    pub fn refresh_new(&mut self) {
        let accel = self.net_acc;
        self.net_acc = [0., 0.];
        let tmpx = self.x;
        let tmpy = self.y;
        self.x = 2. * self.x - self.x_p + accel[0] / (REP * REP);
        self.y = 2. * self.y - self.y_p + accel[1] / (REP * REP);
        self.x_p = tmpx;
        self.y_p = tmpy;
    }

    /// Getter for x-coordinate of Planet.
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Getter for y-coordinate of Planet.
    pub fn get_y(&self) -> f64 {
        self.y
    }
}
