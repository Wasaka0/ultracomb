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

use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod editor;
mod ring_buffer;
mod audio_utility;
mod biquad_filter;
mod even_butterworth;
mod frequency_shifter;

const MAX_DELAY_TIME: f32 = 500.0;
const STRRENGTH_SCALE: f32 = 0.005;
const FREQ_SHIFT_SCALE: f32 = 0.05;
const PHASE_FREQ_SCALE: f32 = 20000.0;
const PHASE_Q_SCALE: f32 = 30.0;

struct Ultracomb {
    params: Arc<UltracombParams>,
    all_pass_cascade: Vec<biquad_filter::BiquadCascade>,
    wet_delay_buffers: Vec<ring_buffer::RingBuffer>,
    dry_delay_buffers: Vec<ring_buffer::RingBuffer>,
    freq_shifters: Vec<frequency_shifter::FrequencyShifter>,
    sampling_frequency: f32,
    editor_state: Arc<ViziaState>
}

#[derive(Params)]
struct UltracombParams {
    #[id = "strength"]
    pub strength: FloatParam,
    #[id = "phasing"]
    pub phasing: FloatParam,
    #[id = "flanging"]
    pub flanging: FloatParam,
    #[id = "chaos"]
    pub chaos: FloatParam,
    #[id = "speed"]
    pub speed: FloatParam,
}

impl Default for Ultracomb {
    fn default() -> Self {
        Self {
            params: Arc::new(UltracombParams::default()),
            all_pass_cascade: Default::default(),
            wet_delay_buffers: Default::default(),
            dry_delay_buffers: Default::default(),
            freq_shifters: Default::default(),
            sampling_frequency: Default::default(),
            editor_state: editor::default_state()
        }
    }
}

impl Default for UltracombParams {
    fn default() -> Self {
        Self {
            strength: FloatParam::new(
                "Strength",
                100.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: 100.0,
                    factor: FloatRange::skew_factor(-1.0)
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_step_size(0.1),
            phasing: FloatParam::new(
                "Phasing",
                0.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-2.5)
                },
            )
            .with_smoother(SmoothingStyle::Linear(150.0)),
            flanging: FloatParam::new(
                "Flanging",
                0.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: 100.0,
                    factor: FloatRange::skew_factor(-2.0)
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_step_size(0.001),
            chaos: FloatParam::new(
                "Chaos",
                0.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: 100.0,
                    factor: FloatRange::skew_factor(-2.0)
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_step_size(0.001),
            speed: FloatParam::new(
                "Speed",
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(100.0))
        }
    }
}

impl Plugin for Ultracomb {
    const NAME: &'static str = "Ultracomb";
    const VENDOR: &'static str = "Wasaka";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
                self.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let num_output_channels = _audio_io_layout
            .main_output_channels
            .expect("Plugin does not have a main output")
            .get() as usize;
        //Create ring buffers
        self.wet_delay_buffers = Vec::new();
        self.dry_delay_buffers = Vec::new();
        self.freq_shifters = Vec::new();
        self.sampling_frequency = _buffer_config.sample_rate;
        for _n in 0..num_output_channels{
            // Initialize Ring Buffers
            let mut wet = ring_buffer::RingBuffer::default();
            wet.resize(_buffer_config.sample_rate, MAX_DELAY_TIME);
            self.wet_delay_buffers.push(wet);
            let mut dry = ring_buffer::RingBuffer::default();
            dry.resize(_buffer_config.sample_rate, MAX_DELAY_TIME);
            self.dry_delay_buffers.push(dry);
            // All-pass filters
            let mut pass = biquad_filter::BiquadCascade::default();
            pass.initialize(biquad_filter::Order::Thirty);
            self.all_pass_cascade.push(pass);
            // Initialize Frequency Shifters 
            let mut shifter = frequency_shifter::FrequencyShifter::default();
            shifter.initialize(_buffer_config.sample_rate);
            self.freq_shifters.push(shifter);
        }
        true
    }

    fn reset(&mut self) {
        for buffer in self.wet_delay_buffers.iter_mut(){
            buffer.reset();
        }
        for buffer in self.dry_delay_buffers.iter_mut(){
            buffer.reset();
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        //Loop for each sample
        for mut sample_per_channel in buffer.iter_samples() {
            // Parameter smoothing happens per sample
            let dry_delay = self.params.chaos.smoothed.next();
            let delay = self.params.flanging.smoothed.next();
            let phase = self.params.phasing.smoothed.next().sqrt();
            let phase_freq = 20050.0 - phase * PHASE_FREQ_SCALE;
            let phase_q = 30.05 - phase * PHASE_Q_SCALE;
            let strength = self.params.strength.smoothed.next() * STRRENGTH_SCALE;
            let freq_shift = self.params.speed.smoothed.next() * FREQ_SHIFT_SCALE;
            //Loop for each channel
            for ((((sample,wet_buffer),shifter),dry_buffer),all_pass) in sample_per_channel.iter_mut().zip(self.wet_delay_buffers.iter_mut()).zip(self.freq_shifters.iter_mut()).zip(self.dry_delay_buffers.iter_mut()).zip(self.all_pass_cascade.iter_mut()){
                wet_buffer.set_delay_ms(delay);
                dry_buffer.set_delay_ms(dry_delay);
                shifter.set_frequency(freq_shift);
                all_pass.all_pass(self.sampling_frequency, phase_freq,phase_q);
                let mut wet = wet_buffer.process(*sample);
                wet = all_pass.process(wet);
                wet = shifter.process(wet);
                *sample = audio_utility::process_linear_dry_wet(dry_buffer.process(*sample),wet,strength);
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Ultracomb {
    const CLAP_ID: &'static str = "com.your-domain.ultracomb";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Ultracomb is a combination of flanging and phasing with continous movement in the frequency spectrum. The rendering pipeline for this was described and named by artist Au5.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for Ultracomb {
    const VST3_CLASS_ID: [u8; 16] = *b"UltracombWsk0000";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Modulation];
}

nih_export_clap!(Ultracomb);
nih_export_vst3!(Ultracomb);
