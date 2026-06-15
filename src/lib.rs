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

use nice_plug::prelude::*;
use vizia_plug::ViziaState;
use std::sync::Arc;

mod editor;
mod audio;
mod ultracomb;
mod custom_param_slider;

const STRENGTH_SCALE: f32 = 0.01;
const MAX_FREQ_SHIFT: f32 = 30.0;
const MIN_FILTER_FREQ: f32 = 30.0;
const MAX_FILTER_FREQ: f32 = 22000.0;

struct Ultracomb {
    params: Arc<UltracombParams>,
    ultracomb: Vec<ultracomb::Ultracomb>,
    crossover: Vec<audio::three_way_crossover::ThreeWayCrossover>,
    gain: Vec<audio::gain_compensation::GainCompensation>,
    pub fx_settings: ultracomb::Settings,
    sampling_frequency: f32,
    phaser_max_freq: f32,
    filter_max_freq: f32,
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
    #[id = "multiplier"]
    pub multiplier: FloatParam,
    #[id = "low-cut"]
    pub low: FloatParam,
    #[id = "high-cut"]
    pub high: FloatParam,
}

impl Default for Ultracomb {
    fn default() -> Self {
        Self {
            params: Arc::new(UltracombParams::default()),
            ultracomb: Default::default(),
            crossover: Default::default(),
            gain: Default::default(),
            fx_settings: Default::default(),
            sampling_frequency: Default::default(),
            phaser_max_freq: Default::default(),
            filter_max_freq: Default::default(),
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
            .with_step_size(0.1)
            .with_unit(" %"),
            phasing: FloatParam::new(
                "Phasing",
                0.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: 100.0,
                    factor: FloatRange::skew_factor(0.0)
                },
            )
            .with_smoother(SmoothingStyle::Linear(100.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_unit(" %"),
            flanging: FloatParam::new(
                "Flanging",
                0.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: ultracomb::MAX_DELAY_TIME,
                    factor: FloatRange::skew_factor(-1.5)
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(3))
            .with_unit(" ms"),
            chaos: FloatParam::new(
                "Chaos",
                0.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: ultracomb::MAX_DELAY_TIME,
                    factor: FloatRange::skew_factor(-1.5)
                },
            )
            .with_value_to_string(formatters::v2s_f32_rounded(3))
            .with_unit(" ms"),
            speed: FloatParam::new(
                "Speed",
                0.0,
                FloatRange::Linear {
                    min: -MAX_FREQ_SHIFT,
                    max: MAX_FREQ_SHIFT,
                },
            )
            .with_step_size(0.1)
            .with_smoother(SmoothingStyle::Linear(100.0))
            .with_unit(" Hz"),
            multiplier: FloatParam::new(
                "Multiplier",
                1.0,
                FloatRange::Linear { min: 1.0, max: ultracomb::MAX_STACK as f32}
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" times")
            .with_step_size(0.05),
            low: FloatParam::new(
                "Low-Cut",
                MIN_FILTER_FREQ,
                FloatRange::Skewed { min: MIN_FILTER_FREQ, max: MAX_FILTER_FREQ, factor: FloatRange::skew_factor(-2.0)}
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(1))
            .with_unit(" Hz"),
            high: FloatParam::new(
                "High-Cut",
                MAX_FILTER_FREQ,
                FloatRange::Skewed { min: MIN_FILTER_FREQ, max: MAX_FILTER_FREQ, factor: FloatRange::skew_factor(-2.0)}
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(1))
            .with_unit(" Hz")
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
        self.sampling_frequency = _buffer_config.sample_rate;
        self.phaser_max_freq = (self.sampling_frequency/2.0) - 1000.0;
        self.filter_max_freq = self.sampling_frequency/2.0;
        //Create effect for each channel
        self.ultracomb = Vec::new();
        for _n in 0..num_output_channels{
            let mut channel: ultracomb::Ultracomb = Default::default();
            channel.initialize(self.sampling_frequency);
            self.ultracomb.push(channel);

            let mut cross: audio::three_way_crossover::ThreeWayCrossover = Default::default();
            cross.initialize(self.sampling_frequency);
            self.crossover.push(cross);

            let mut gain: audio::gain_compensation::GainCompensation = Default::default();
            gain.initialize(self.sampling_frequency);
            self.gain.push(gain);
        }
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Sanitize input
        for mut sample_per_channel in buffer.iter_samples() {
            for sample in sample_per_channel.iter_mut(){
                *sample = audio::utility::clean_sample(*sample);
            }
        }
        //Loop for each sample
        for mut sample_per_channel in buffer.iter_samples() {
            // Parameter smoothing happens per sample
            self.fx_settings.dry_delay = self.params.chaos.smoothed.next();
            self.fx_settings.delay = self.params.flanging.smoothed.next();
            let phase = self.params.phasing.smoothed.next();
            self.fx_settings.phaser_freq = if phase < 10.0 {self.phaser_max_freq - (((self.phaser_max_freq - 10000.0) / 10.0) * phase)} else if phase < 30.0 { 10000.0 - (phase - 10.0) * 250.0} else if phase < 80.0 { 5000.0 - (phase - 30.0) * 70.0}  else {1500.0 - (phase - 80.0) * 25.0};
            self.fx_settings.phaser_q = if phase < 10.0 {100.0 - 9.5 * phase} else if phase < 50.0 {5.0 - (phase - 10.0) * 0.075} else if phase < 70.0 { 2.0 - (phase - 50.0) * 0.05} else {1.0 - (phase - 70.0) * 0.02};
            let strength = self.params.strength.smoothed.next() * STRENGTH_SCALE;
            self.fx_settings.freq_shift = self.params.speed.smoothed.next();
            self.fx_settings.multiplier = self.params.multiplier.smoothed.next();
            let low_cut = self.params.low.smoothed.next();
            let high_cut = self.params.high.smoothed.next();
            //Loop for each channel
            for (((sample, ultracomb), crossover), gain) in sample_per_channel.iter_mut().zip(self.ultracomb.iter_mut()).zip(self.crossover.iter_mut()).zip(self.gain.iter_mut()){
                ultracomb.set_settings(self.fx_settings);
                crossover.set_frequencies(low_cut, high_cut.clamp(MIN_FILTER_FREQ,self.filter_max_freq));
                let bands = crossover.process(*sample);
                gain.write_pre(bands.1);
                let mut wet = ultracomb.process(bands.1);
                gain.write_post(wet);
                wet *= gain.get_gain();
                *sample = audio::utility::process_linear_dry_wet(bands.1,wet,strength) + bands.0 + bands.2;
            }
        }
        // Sanitize output
        for mut sample_per_channel in buffer.iter_samples() {
            for sample in sample_per_channel.iter_mut(){
                *sample = audio::utility::clean_sample(*sample);
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

nice_export_clap!(Ultracomb);
nice_export_vst3!(Ultracomb);
