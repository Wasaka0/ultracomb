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
// A frequency shifter using the quadrature oscillator method  
#[derive(Clone, Debug, Default)]
pub struct FrequencyShifter{
    low_pass_filters: [BiquadFilter; 2],
    upper_sample: f32,
    lower_sample: f32,
}

impl FrequencyShifter{
    //Process a single sample ruturning the output shifted in frequency 
    pub fn process(&mut self, sample: f32) -> f32 {
        // Upper branch of the frequency shifter
        self.upper_sample = self.low_pass_filters[0].process(sample);
        // Lower branch of the frequency shifter
        self.lower_sample = self.low_pass_filters[1].process(sample);

        self.upper_sample
    }

    //Prepares the shifter by configuring its elements according to the given sample frequency
    pub fn initialize(&mut self, sample_rate: f32) {
        for filter in &mut self.low_pass_filters{
            filter.reset();
            filter.low_pass(sample_rate,sample_rate / 4.0,1.0);
        }
    }

    //Resets the state of the frequency shifter
    pub fn reset(&mut self) {
    }
}
