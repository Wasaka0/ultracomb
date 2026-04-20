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

use crate::audio::*;

pub const MAX_DELAY_TIME: f32 = 15.0;
const MAX_STACK: usize = 16;

#[derive(Clone, Debug, Default)]
pub struct Effect{
    chain: [EffectChain; MAX_STACK],
    settings: Settings,
    sample: f32
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Settings{
    pub dry_delay: f32,
    pub delay: f32,
    pub phaser_freq: f32,
    pub phaser_q: f32,
    pub freq_shift: f32,
    pub multiplier: f32,
}

#[derive(Clone, Debug, Default)]
struct EffectChain{
    all_pass: biquad_filter::BiquadCascade,
    wet_buffer: delay::Delay,
    dry_buffer: delay::Delay,
    freq_shifter: frequency_shifter::FrequencyShifter,
    sample_rate: f32,
}

impl EffectChain{
    pub fn initialize(&mut self, sample_rate: f32){
        self.sample_rate = sample_rate;
        //Initialize ring buffers
        self.wet_buffer = Default::default();
        self.wet_buffer.resize(self.sample_rate, MAX_DELAY_TIME);
        self.dry_buffer = Default::default();
        self.dry_buffer.resize(self.sample_rate, MAX_DELAY_TIME);
        // All-pass filters
        self.all_pass = Default::default();
        self.all_pass.initialize(biquad_filter::Order::Thirty);
        // Initialize Frequency Shifters 
        self.freq_shifter = Default::default();
        self.freq_shifter.initialize(self.sample_rate);
    }
    pub fn process(&mut self, sample: f32, settings: Settings) -> f32{
        //Configure elements
        self.wet_buffer.set_delay_ms(settings.delay);
        self.dry_buffer.set_delay_ms(settings.dry_delay);
        self.freq_shifter.set_frequency(settings.freq_shift);
        self.all_pass.all_pass(self.sample_rate, settings.phaser_freq, settings.phaser_q);
        //Process audio
        let mut wet = self.wet_buffer.process(sample);
        wet = self.all_pass.process(wet);
        wet = self.freq_shifter.process(wet);
        wet = 0.5 * (self.dry_buffer.process(sample) + wet);
        wet
    }
}

impl Effect{
    pub fn initialize(&mut self, sample_rate: f32){
        for effect in &mut self.chain{
            effect.initialize(sample_rate);
        }
    }
    pub fn process(&mut self, sample: f32) -> f32{
        self.sample = sample;
        let last_full_chain = self.settings.multiplier.trunc() as usize;
        let next_chain_ratio = self.settings.multiplier.fract();
        for i in 0..last_full_chain{
            self.sample = self.chain[i].process(self.sample,self.settings);
        }
        if last_full_chain < MAX_STACK && next_chain_ratio > 0.0{
            self.sample = (1.0 - next_chain_ratio) * self.sample + next_chain_ratio * self.chain[last_full_chain].process(self.sample,self.settings);
        }
        self.sample
    }
    pub fn set_settings(&mut self, new_settings: Settings){
        self.settings = new_settings;
    }
}