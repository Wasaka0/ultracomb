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

use crate::biquad_filter::BiquadFilter;

// Reducing this will reduce oscillator artifacts at the cost of memory.
const LUT_BASE_FREQ: f32 = 3.0;

// A frequency shifter using the quadrature oscillator method  
#[derive(Clone, Debug, Default)]
pub struct FrequencyShifter{
    low_pass_filters: [BiquadFilter; 2],
    upper_sample: f32,
    lower_sample: f32,
    first_osc: QuadratureOscillator,
    second_osc: QuadratureOscillator,
    freq_static_osc: f32
}

// An sine oscillator that provides two outputs with a difference of 90 degrees
#[derive(Clone, Debug, Default)]
pub struct QuadratureOscillator{
    lut: Vec<f32>,
    sin_index: usize,
    cos_index: usize,
    cos_offset: f32,
    phase: f32,
    step: f32,
}

impl QuadratureOscillator{
    // Initilize the oscillator, filling the look-up table
    pub fn initialize(&mut self, sample_rate: f32){
        let base_period = 1.0 / LUT_BASE_FREQ;
        let lut_length = (sample_rate * base_period).floor() as usize;
        self.lut.resize(lut_length, 0.0);
        // Fill LUT with one period of 20Hz sine wave
        for i in 0..self.lut.len(){
            let t = i as f32 / sample_rate;
            self.lut[i] = (consts::TAU * LUT_BASE_FREQ * t).sin();
        }

        // Calculate cosine offset
        self.cos_offset = (sample_rate * base_period) / 4.0;
        
        self.sin_index = 0;
        self.cos_index = self.cos_offset.floor() as usize;
        self.phase = 0.0;
        self.step = 1.0;
    }

    // Returns the current sin and cos sample
    pub fn next(&mut self) -> (f32,f32){
        // Read sin and cos
        let sin = self.lut[self.sin_index];
        let cos = self.lut[self.cos_index];

        // Advance phase
        self.phase = self.phase + self.step;
        if self.phase >= self.lut.len() as f32{
            self.phase = self.phase - self.lut.len() as f32;
        }
        // Update indexes
        self.sin_index = self.phase.floor() as usize;
        self.cos_index = (self.phase + self.cos_offset).floor() as usize;
        if self.cos_index >= self.lut.len(){
            self.cos_index = self.cos_index - self.lut.len();
        }
        (sin,cos)
    }

    // Sets the oscillator frequency
    pub fn set_frequency(&mut self, frequency: f32){
        self.step = frequency / LUT_BASE_FREQ;
    }
}

impl FrequencyShifter{
    //Process a single sample ruturning the output shifted in frequency 
    pub fn process(&mut self, sample: f32) -> f32 {
        // Get quadrature samples
        let (sin_1, cos_1) = self.first_osc.next();
        let (sin_2, cos_2) = self.second_osc.next();

        // Upper branch of the frequency shifter
        self.upper_sample = sample * sin_1;
        self.upper_sample = self.low_pass_filters[0].process(self.upper_sample);
        self.upper_sample = self.upper_sample * sin_2;
        // Lower branch of the frequency shifter
        self.lower_sample = sample * cos_1;
        self.lower_sample = self.low_pass_filters[1].process(self.lower_sample);
        self.lower_sample = self.lower_sample * cos_2;

        self.lower_sample + self.upper_sample
    }

    //Prepares the shifter by configuring its elements according to the given sample frequency
    pub fn initialize(&mut self, sample_rate: f32) {
        for filter in &mut self.low_pass_filters{
            filter.reset();
            filter.low_pass(sample_rate,sample_rate / 4.0,1.0);
        }

        self.first_osc.initialize(sample_rate);
        self.first_osc.set_frequency(self.freq_static_osc);
        self.second_osc.initialize(sample_rate);
        self.second_osc.set_frequency(self.freq_static_osc);
    }

    //Resets the state of the frequency shifter
    pub fn reset(&mut self) {
    }

    pub fn set_frequency(&mut self, frequency: f32){
        self.second_osc.set_frequency(self.freq_static_osc + frequency);
    }
}
