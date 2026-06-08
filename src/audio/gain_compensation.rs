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

const MEASUREMENT_TIME: f32 = 0.050;

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
    sum: f32
}

impl CompState{
    pub fn write(&mut self, sample: f32){
        self.sum -= self.samples[self.index];
        self.samples[self.index] = sample * sample;
        self.sum += self.samples[self.index];
        self.index += 1;
        if self.index == self.samples.len() {
            self.index = 0;
        }
    }

    pub fn initialize(&mut self, length: usize){
        self.samples = vec![0.0; length];
        self.sum = 0.0;
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
        let pre_rms = (self.pre.sum / self.num_samples as f32).sqrt();
        let post_rms = (self.post.sum / self.num_samples as f32).sqrt();
        pre_rms / post_rms
    }
}