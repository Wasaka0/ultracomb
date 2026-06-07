// Ultracomb
// Copyright (C) 2026 M. M. Trinidad
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see https://www.gnu.org/licenses/.

use crate::audio::biquad_filter;

// Butterworth filter of even order build with biquad cascade
#[derive(Clone,Debug, Default)]
pub struct Butterworth{
    cascade: biquad_filter::BiquadCascade,
    order: u32
}

impl Butterworth{
    pub fn process(&mut self, sample: f32) -> f32 {
        self.cascade.process(sample)
    }

    pub fn initialize(&mut self, order: u32){
        self.order = order;
        self.cascade.initialize(order);
    }

    pub fn low_pass(&mut self, sampling_frequency: f32, center_frequency: f32) {
        let q = Self::get_q(self.order);
        self.cascade.low_pass(sampling_frequency, center_frequency, q);
    }

    fn get_q(order: u32) -> Vec<f32>{
        match order{
            2 => {
                return vec![0.70710678];
            }
            4 => {
                return vec![0.54119610, 1.3065630];
            }
            6 => {
                return vec![0.51763809, 0.70710678, 1.9318517];
            }
            _ => {
                panic!();
            }
        }
    }
}
