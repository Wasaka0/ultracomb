# Ultracomb

Ultracomb is a VST3/CLAP plugin that implements an audio effect chain described by the artist Au5 [here](https://www.youtube.com/watch?v=_SyB2WqKwP4).

## Audio details
The block diagram for the effect looks like this:
![Block diagram of the Ultracomb audio effect](img/Ultracomb-block-diagram.png)

The phaser has 15 notches (all-pass filter of order 30)
The frequency shifter is implemented using "The third method" discover by Donald k. Weaver.

## Download
You can find CLAP and VST3 binaries for Linux, Mac and Windows in the [releases page](https://github.com/Wasaka0/ultracomb/releases).
Latest version: 0.2.0 [link](https://github.com/Wasaka0/ultracomb/releases/tag/0.2.0)
## Building

After installing [Rust](https://rustup.rs/), you can compile Ultracomb as follows:

```shell
cargo xtask bundle ultracomb --release
```

## To-do
- [X] Audio processing
    - [x] Flanging
        - [x] Interpolation for delays between samples
        - [ ] Apply interpolation only when modifying delay
    - [x] Phasing
        - [x] All pass filter
        - [ ] Variable number of notches
    - [x] Frequency Shifter
        - [x] Low pass filter
            - [x] Changed to elliptic filter, which improves shifter output by attenuating more the unwanted frequency band with its steeper transition slope 
        - [x] Quadrature oscillator
        - [x] Fade-in and out
        - [x] Try other frequency shifting methods e.g. Hilbert Filter
            - [x] Tried without clear improvement right away
        - [ ] Solve problems with DC, pre filtering might be enough
    - [ ] Gain compensation
        - [x] For Frequency shifter
        - [ ] For Flanger/Phaser only
    - [x] Effect multiplier
- [ ] CD
    - [x] Windows
    - [x] Linux
    - [x] MacOS
    - [ ] Automatic releases
    - [x] Unify CI script 
    - [x] Add version to artifact name
- [x] GUI
    - [ ] Add knobs instead of sliders
    - [ ] Maybe switch to vizia-plug which is more recently updated than nih_plug_vizia 
- [ ] Plug-in parameter definition
- [ ] Performance
    - [ ] Design benchmarks
    - [ ] Improve performance
