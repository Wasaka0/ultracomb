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

//Measurement time equals half a cycle of the lowest frequency the effect takes in 30Hz
const MEASUREMENT_TIME: f32 = 0.016;

// 4th order Three way crossover filter
#[derive(Clone,Debug, Default)]
pub struct GainCompensation{
    pre: CompState,
    post: CompState,
    num_samples: usize
}

#[derive(Clone,Debug, Default)]
struct CompState{
    pub samples: Vec<f32>,
    index: usize,
    max: f32,
    min: f32,
}

impl CompState{
    pub fn write(&mut self, sample: f32){
        //Check if new sample is min or max
        if sample > self.max {
            self.max = sample;
        } else if sample < self.min {
            self.min = sample;
        }
        //Store new sample
        if self.samples[self.index] == self.max {
            self.samples[self.index] = sample;
            self.max = self.samples.iter().cloned().fold(0./0., f32::max);
        } else if self.samples[self.index] == self.min {
            self.samples[self.index] = sample;
            self.min = self.samples.iter().cloned().fold(1./0. /* inf */, f32::min);
        } else {
            self.samples[self.index] = sample;
        }
        //Move index
        self.index += 1;
        if self.index == self.samples.len() {
            self.index = 0;
        }
    }

    pub fn initialize(&mut self, length: usize){
        self.samples = vec![0.0; length];
        self.max = -f32::INFINITY;
        self.min = f32::INFINITY;
        self.index = 0;
    }

}

impl GainCompensation{
    pub fn initialize(&mut self, sample_rate: f32){
        let sample_length = MEASUREMENT_TIME * sample_rate;
        self.num_samples = sample_length.floor() as usize;
        self.pre.initialize(self.num_samples);
        self.post.initialize(self.num_samples);
    }

    pub fn write_pre(&mut self, sample: f32){
        self.pre.write(sample);
    }

    pub fn write_post(&mut self, sample: f32){
        self.post.write(sample);
    }

    pub fn get_gain(&self) -> f32 {
        ((self.pre.max - self.pre.min)/(self.post.max - self.post.min)).clamp(0.0, 8.0)
    }
}