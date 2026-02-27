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

use crate::biquad_filter::BiquadFilter;

// Butterworth filter of even order build with biquad cascade
#[derive(Clone,Debug, Default)]
pub struct EvenButterworth{
    biquads: Vec<BiquadFilter>,
    sample: f32,
    order: Order
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Order{
    #[default]
    Second,
    Forth,
    Sixth,
}

impl EvenButterworth{
    pub fn process(&mut self, sample: f32) -> f32 {
        self.sample = sample;
        for filter in &mut self.biquads{
            self.sample = filter.process(self.sample);
        }
        self.sample
    }

    pub fn initialize(&mut self, order: Order){
        self.order = order;
        match order{
            Order::Second => {
                self.biquads = Vec::new();
                self.biquads.push(BiquadFilter::default());
            }
            Order::Forth => {
                self.biquads = Vec::new();
                self.biquads.push(BiquadFilter::default());
                self.biquads.push(BiquadFilter::default());
            }
            Order::Sixth => {
                self.biquads = Vec::new();
                self.biquads.push(BiquadFilter::default());
                self.biquads.push(BiquadFilter::default());
                self.biquads.push(BiquadFilter::default());
            }
        }
    }

    pub fn low_pass(&mut self, sampling_frequency: f32, center_frequency: f32) {
        let q = Self::get_q(self.order);
        for (filter, q) in self.biquads.iter_mut().zip(q){
            filter.reset();
            filter.low_pass(sampling_frequency, center_frequency, q);
        }
    }

    fn get_q(order: Order) -> Vec<f32>{
        match order{
            Order::Second => {
                return vec![0.70710678];
            }
            Order::Forth => {
                return vec![0.54119610, 1.3065630];
            }
            Order::Sixth => {
                return vec![0.51763809, 0.70710678, 1.9318517];
            }
        }
    }

}
