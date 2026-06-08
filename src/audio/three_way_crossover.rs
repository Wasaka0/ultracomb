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

use crate::audio::butterworth_filter::Butterworth;

// 4th order Three way crossover filter
#[derive(Clone,Debug, Default)]
pub struct ThreeWayCrossover{
    lo: [Butterworth; 4],
    mid: [Butterworth; 4],
    hi: [Butterworth; 2],
    low_freq: f32,
    high_freq: f32,
    sampling_frequency: f32,
    samples: (f32,f32,f32)
}

impl ThreeWayCrossover{
    pub fn process(&mut self, sample: f32) -> (f32, f32, f32) {
        self.samples.0 = self.lo[0].process(sample);
        self.samples.0 = self.lo[1].process(self.samples.0);
        self.samples.0 = self.lo[2].process(self.samples.0);
        self.samples.0 = self.lo[3].process(self.samples.0);

        self.samples.1 = self.mid[0].process(sample);
        self.samples.1 = self.mid[1].process(self.samples.1);

        self.samples.2 = self.hi[0].process(self.samples.1);
        self.samples.2 = self.hi[1].process(self.samples.2);

        self.samples.1 = self.mid[2].process(self.samples.1);
        self.samples.1 = self.mid[3].process(self.samples.1);

        self.samples
    }

    pub fn initialize(&mut self, sampling_frequency: f32){
        for filter in self.lo.iter_mut() {
            filter.initialize(2);
        }
        for filter in self.mid.iter_mut() {
            filter.initialize(2);
        }
        for filter in self.hi.iter_mut(){
            filter.initialize(2);
        }
        self.sampling_frequency = sampling_frequency;
    }

    pub fn set_frequencies(&mut self, low_cut: f32, high_cut: f32){
        self.low_freq = low_cut;
        if low_cut > high_cut{
            self.high_freq = low_cut;
        } else{
            self.high_freq = high_cut;
        }

        self.lo[0].low_pass(self.sampling_frequency, self.low_freq);
        self.lo[1].low_pass(self.sampling_frequency, self.low_freq);
        self.lo[2].low_pass(self.sampling_frequency, self.high_freq);
        self.lo[3].low_pass(self.sampling_frequency, self.high_freq);

        self.mid[0].high_pass(self.sampling_frequency, self.low_freq);
        self.mid[1].high_pass(self.sampling_frequency, self.low_freq);
        self.mid[2].low_pass(self.sampling_frequency, self.high_freq);
        self.mid[3].low_pass(self.sampling_frequency, self.high_freq);

        self.hi[0].high_pass(self.sampling_frequency, self.high_freq);
        self.hi[1].high_pass(self.sampling_frequency, self.high_freq);
    }

}