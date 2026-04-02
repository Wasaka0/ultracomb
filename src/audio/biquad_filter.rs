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

use std::f32::consts;

// A biquad filter that provides multiple filter types 
#[derive(Clone, Debug, Default)]
pub struct BiquadFilter{
    coefficients: BiquadCoefficients,
    samples: SampleStorage
}

// Coefficients used to process the filter output
#[derive(Clone, Debug, Default)]
struct BiquadCoefficients{
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32
}
impl BiquadCoefficients {
    pub fn identity(&mut self){
        self.b0 = 1.0;
        self.b1 = 0.0;
        self.b2 = 0.0;
        self.a1 = 0.0;
        self.a2 = 0.0;
    }
}

// Samples that need to be recalled during filtering process
#[derive(Clone, Debug, Default)]
struct SampleStorage{
    x1: f32,
    x2: f32,
    y0: f32,
    y1: f32,
    y2: f32
}
impl SampleStorage {
    pub fn reset(&mut self){
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y0 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

impl BiquadFilter {
    //Process a single input sample returning the filter output
    pub fn process(&mut self, sample: f32) -> f32 {
        self.samples.y0 = self.coefficients.b0 * sample + self.coefficients.b1 * self.samples.x1 + self.coefficients.b2 * self.samples.x2 - self.coefficients.a1 * self.samples.y1 - self.coefficients.a2 * self.samples.y2;
        self.samples.x2 = self.samples.x1;
        self.samples.x1 = sample;
        self.samples.y2 = self.samples.y1;
        self.samples.y1 = self.samples.y0;
        self.samples.y0
    }

    //Resets the state of the filter
    pub fn reset(&mut self) {
        self.coefficients.identity();
        self.samples.reset();
    }

    //Calculates the coefficients for a low pass filter without checking for 
    //filter stability. This may produce unstable filter coefficients.
    pub fn low_pass(&mut self, sampling_frequency: f32, center_frequency: f32, q: f32) {
        let w0 = consts::TAU * (center_frequency / sampling_frequency);
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();
        
        let a0 = 1.0 + alpha;
        
        let b0 = (1.0 - cos_w0) / (2.0 * a0);
        let b1 = (1.0 - cos_w0) / a0;
        let b2 = b0;
        
        let a1 = (-2.0 * cos_w0) / a0;
        let a2 = (1.0 - alpha) / a0;
        self.coefficients = BiquadCoefficients { b0, b1, b2, a1, a2};
    }

    //Calculates the coefficients for an all pass filter.
    pub fn all_pass(&mut self, sampling_frequency: f32, center_frequency: f32, q: f32) {
        let w0 = consts::TAU * (center_frequency / sampling_frequency);
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();
        
        let a0 = 1.0 + alpha;
        
        let b0 = (1.0 - alpha) / a0;
        let b1 = (-2.0 * cos_w0) / a0;
        let b2 = 1.0;
        
        let a1 = b1;
        let a2 = b0;
        self.coefficients = BiquadCoefficients { b0, b1, b2, a1, a2};
    }
}

#[derive(Clone,Debug, Default)]
pub struct BiquadCascade{
    biquads: Vec<BiquadFilter>,
    sample: f32
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Order{
    #[default]
    Second,
    Forth,
    Sixth,
    Thirty
}

impl BiquadCascade {
    //Process a single input sample returning the filter output
    pub fn process(&mut self, sample: f32) -> f32 {
        self.sample = sample;
        for filter in &mut self.biquads{
            self.sample = filter.process(self.sample);
        }
        self.sample
    }

    //Resets the state of the filter
    pub fn reset(&mut self) {
        for filter in &mut self.biquads{
            filter.coefficients.identity();
            filter.samples.reset();
        }
        self.sample = 0.0;
    }

    pub fn initialize(&mut self, order: Order){
        let n = match order{
            Order::Second => {
                1
            }
            Order::Forth => {
                2
            }
            Order::Sixth => {
                3
            }
            Order::Thirty => {
                15
            }
        };
        self.biquads = Vec::new();
        for _i in 0..n{
            let mut filter = BiquadFilter::default();
            filter.reset();
            self.biquads.push(filter);
        }
    }

    // Calculates the coefficients for a low pass filter cascade all with the same cut-off frequency
    // but with individual q. Size of vector q must at least half the order given at initialize.
    pub fn low_pass(&mut self, sampling_frequency: f32, center_frequency: f32, q: Vec<f32>) {
        for (filter, q) in self.biquads.iter_mut().zip(q){
            filter.reset();
            filter.low_pass(sampling_frequency, center_frequency, q);
        }
    }

    //Calculates the coefficients for an all pass filter. This may produce unstable filter coefficients.
    pub fn all_pass(&mut self, sampling_frequency: f32, center_frequency: f32, q: f32) {
        for filter in self.biquads.iter_mut(){
            filter.all_pass(sampling_frequency, center_frequency, q);
        }
    }
}

