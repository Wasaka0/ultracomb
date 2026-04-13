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
mod audio;
mod ultracomb;

const STRENGTH_SCALE: f32 = 0.01;
const MAX_FREQ_SHIFT: f32 = 30.0;

struct Ultracomb {
    params: Arc<UltracombParams>,
    ultracomb: Vec<ultracomb::Effect>,
    pub fx_settings: ultracomb::Settings,
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
    #[id = "multiplier"]
    pub multiplier: FloatParam,
}

impl Default for Ultracomb {
    fn default() -> Self {
        Self {
            params: Arc::new(UltracombParams::default()),
            ultracomb: Default::default(),
            fx_settings: Default::default(),
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
            .with_smoother(SmoothingStyle::Linear(50.0))
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
            .with_smoother(SmoothingStyle::Linear(100.0))
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
            .with_smoother(SmoothingStyle::Linear(100.0))
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
                FloatRange::Linear { min: 1.0, max: 16.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" times")
            .with_step_size(0.05)
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
        //Create effect for each channel
        self.ultracomb = Vec::new();
        for _n in 0..num_output_channels{
            let mut channel: ultracomb::Effect = Default::default();
            channel.initialize(self.sampling_frequency);
            self.ultracomb.push(channel);
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
        //Loop for each sample
        for mut sample_per_channel in buffer.iter_samples() {
            // Parameter smoothing happens per sample
            self.fx_settings.dry_delay = self.params.chaos.smoothed.next();
            self.fx_settings.delay = self.params.flanging.smoothed.next();
            let phase = self.params.phasing.smoothed.next();
            self.fx_settings.phaser_freq = if phase < 10.0 {20000.0 - 1000.0 * phase} else if phase < 30.0 { 10000.0 - (phase - 10.0) * 250.0} else if phase < 70.0 { 5000.0 - (phase - 30.0) * 100.0}  else {1000.0 - (phase - 70.0) * 30.0};
            self.fx_settings.phaser_q = if phase < 10.0 {30.0 - 2.5 * phase} else if phase < 50.0 {5.0 - (phase - 10.0) * 0.075} else if phase < 70.0 { 2.0 - (phase - 50.0) * 0.05} else {1.0 - (phase - 70.0) * 0.0323};
            let strength = self.params.strength.smoothed.next() * STRENGTH_SCALE;
            self.fx_settings.freq_shift = self.params.speed.smoothed.next();
            self.fx_settings.multiplier = self.params.multiplier.smoothed.next();
            //Loop for each channel
            for (sample,ultracomb) in sample_per_channel.iter_mut().zip(self.ultracomb.iter_mut()){
                ultracomb.set_settings(self.fx_settings);
                let wet = ultracomb.process(*sample);                
                *sample = audio::utility::process_linear_dry_wet(*sample,wet,strength);
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
