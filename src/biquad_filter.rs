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

// Samples that need to be recalled during filtering process
#[derive(Clone, Debug, Default)]
struct SampleStorage{
    x1: f32,
    x2: f32,
    y0: f32,
    y1: f32,
    y2: f32
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
    //Calculates the coefficients for a low pass filter without checking for 
    //filter stability. This may produce unstable filter coefficients.
    pub fn low_pass(&mut self, sampling_frequency: f32, center_frequency: f32, Q: f32) {
        let w0 = consts::TAU * (center_frequency / sampling_frequency);
        let alpha = w0.sin() / (2.0 * Q);
        let cos_w0 = w0.cos();
        
        let a0 = 1.0 + alpha;
        
        let b0 = (1.0 - cos_w0) / (2.0 * a0);
        let b1 = (1.0 - cos_w0) / a0;
        let b2 = b0;
        
        let a1 = (-2.0 * cos_w0) / a0;
        let a2 = (1.0 - alpha) / a0;
        self.coefficients = BiquadCoefficients { b0, b1, b2, a1, a2};
    }
}