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
use crate::audio::utility::process_linear_dry_wet;

pub const MAX_DELAY_TIME: f32 = 5.0;
pub const MAX_STACK: usize = 16;
// Length in seconds of crossfade applied when freq shifter is not in use 
const CROSSFADE_LENGTH: f32 = 0.05;

#[derive(Clone, Debug, Default)]
pub struct Ultracomb{
    chain: [EffectChain; MAX_STACK],
    freq_shift_osc: frequency_shifter::FreqShiftOsc,
    freq_shift_fade_ratio: f32,
    freq_shift_fade_step: f32,
    settings: Settings,
    sample: f32,
    sample_rate: f32
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
    shift_fade_ratio: f32
}

impl EffectChain{
    pub fn initialize(&mut self, sample_rate: f32){
        //Initialize ring buffers
        self.wet_buffer = Default::default();
        self.wet_buffer.resize(sample_rate, MAX_DELAY_TIME);
        self.dry_buffer = Default::default();
        self.dry_buffer.resize(sample_rate, MAX_DELAY_TIME);
        // All-pass filters
        self.all_pass = Default::default();
        self.all_pass.initialize(30);
        // Initialize Frequency Shifters 
        self.freq_shifter = Default::default();
        self.freq_shifter.initialize();
    }
    pub fn process(&mut self, sample: f32) -> f32{
        //Process audio
        let mut wet = self.wet_buffer.process(sample);
        wet = self.all_pass.process(wet);
        if self.shift_fade_ratio > 0.0{
            wet = process_linear_dry_wet(wet, self.freq_shifter.process(wet), self.shift_fade_ratio)
        }
        wet = 0.5 * (self.dry_buffer.process(sample) + wet);
        wet
    }
    fn update_state(&mut self, shift_osc_samples: ((f32,f32),(f32,f32)), shift_fade_ratio: f32){
        self.freq_shifter.set_osc_samples(shift_osc_samples);
        self.shift_fade_ratio = shift_fade_ratio;
    }
}

impl Ultracomb{
    pub fn initialize(&mut self, sample_rate: f32){
        self.sample_rate = sample_rate;
        for effect in &mut self.chain{
            effect.initialize(sample_rate);
        }
        self.freq_shift_osc.initialize(sample_rate);
        self.freq_shift_fade_ratio = 0.0;
        self.freq_shift_fade_step = 1.0 / (CROSSFADE_LENGTH * sample_rate);
    }
    pub fn process(&mut self, sample: f32) -> f32{
        //Calculate fade-in or fade-out ratio for the frequency shifters
        if self.settings.freq_shift == 0.0 { // Fade-out
            self.freq_shift_fade_ratio -= self.freq_shift_fade_step;
            self.freq_shift_fade_ratio = self.freq_shift_fade_ratio.clamp(0.0, 1.0);
        } else { // Fade-in
            self.freq_shift_fade_ratio += self.freq_shift_fade_step;
            self.freq_shift_fade_ratio = self.freq_shift_fade_ratio.clamp(0.0, 1.0);
        }

        self.sample = sample;
        let last_full_chain = self.settings.multiplier.trunc() as usize;
        let next_chain_ratio = self.settings.multiplier.fract();
        let shift_osc_samples = self.freq_shift_osc.next();
        // Amplify signal depending on frequency shift, multiplier and dry_delay (Chaos) to compensate level loss when Chaos is set together with this two parameters.
        // When chaos is not active the gain compensation attenuates the output. This allows to limit the amount the compensator amplifies which prevents blowing out when input is a sine signals.
        self.sample *= 1.0 + ((self.settings.freq_shift.abs().clamp(0.0, 13.0) * (self.settings.multiplier - 1.0) * (self.settings.dry_delay)) * 15.0);
        for i in 0..last_full_chain{
            self.chain[i].update_state(shift_osc_samples, self.freq_shift_fade_ratio);
            self.sample = self.chain[i].process(self.sample);
        }
        if last_full_chain < MAX_STACK && next_chain_ratio > 0.0{
            self.chain[last_full_chain].update_state(shift_osc_samples, self.freq_shift_fade_ratio);
            self.sample = process_linear_dry_wet(self.sample,self.chain[last_full_chain].process(self.sample),next_chain_ratio)
        }
        self.sample
    }
    pub fn set_settings(&mut self, new_settings: Settings){
        //Update delays when needed
        if new_settings.delay != self.settings.delay {
            for effect in self.chain.iter_mut() {
                effect.wet_buffer.set_delay_ms(new_settings.delay );
            }
        }
        if new_settings.dry_delay != self.settings.dry_delay {
            for effect in self.chain.iter_mut() {
                effect.dry_buffer.set_delay_ms(new_settings.dry_delay );
            }
        }
        //Update phaser filter when needed
        if new_settings.phaser_freq != self.settings.phaser_freq || new_settings.phaser_q != self.settings.phaser_q{
            for effect in self.chain.iter_mut() {
                effect.all_pass.all_pass(self.sample_rate, new_settings.phaser_freq, new_settings.phaser_q);
            }
        }

        self.settings = new_settings;
        self.freq_shift_osc.set_frequency(self.settings.freq_shift);
    }
}