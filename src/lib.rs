use nih_plug::prelude::*;
use std::sync::Arc;

mod ring_buffer;
mod audio_utility;

const MAX_DELAY_TIME: f32 = 500.0;
const DELAY_SCALE: f32 = 150.0;
const STRRENGTH_SCALE: f32 = 0.005;

struct Ultracomb {
    params: Arc<UltracombParams>,
    delay_buffers: Vec<ring_buffer::RingBuffer>,
}

#[derive(Params)]
struct UltracombParams {
    #[id = "strength"]
    pub strength: FloatParam,
    #[id = "odd"]
    pub odd: FloatParam,
}

impl Default for Ultracomb {
    fn default() -> Self {
        Self {
            params: Arc::new(UltracombParams::default()),
            delay_buffers: Default::default(),
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
            odd: FloatParam::new(
                "Odd",
                0.0,
                FloatRange::Skewed{
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-1.0)
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_step_size(0.001),
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
        self.delay_buffers = Vec::new();
        for n in 0..num_output_channels{
            let buffer = ring_buffer::RingBuffer::default();
            self.delay_buffers.push(buffer);
            self.delay_buffers[n].resize(_buffer_config.sample_rate, MAX_DELAY_TIME);
        }
        true
    }

    fn reset(&mut self) {
        for buffer in self.delay_buffers.iter_mut(){
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
            let delay = self.params.odd.smoothed.next() * DELAY_SCALE;
            let strength = self.params.strength.smoothed.next() * STRRENGTH_SCALE;
            //Loop for each channel
            for (sample,delay_buffer) in sample_per_channel.iter_mut().zip(&mut self.delay_buffers){
                delay_buffer.set_delay_ms(delay);
                let dry = *sample;
                let wet = delay_buffer.process(*sample);
                *sample = audio_utility::process_linear_dry_wet(dry,wet,strength);
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
