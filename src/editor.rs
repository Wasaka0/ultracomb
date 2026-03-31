use nih_plug::prelude::{Editor};
use vizia_plug::vizia::{prelude::*};
use vizia_plug::widgets::*;
use vizia_plug::{ create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::UltracombParams;

#[derive(Lens)]
struct Data {
    params: Arc<UltracombParams>
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 200))
}

pub(crate) fn create(
    params: Arc<UltracombParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        Data {
            params: params.clone(),
        }
        .build(cx);
        // AFAIK this should never fail on runtime. 
        cx.add_stylesheet(include_style!("src/style.css")).expect("Style sheet could not be opened");
        // UI Elements
        // Top Text
        HStack::new(cx, |cx|{
            Label::new(cx, "Ultracomb")
                .font_weight(FontWeightKeyword::Bold)
                .font_size(25.0)
                .left(Pixels(7.0));
            Label::new(cx, env!("CARGO_PKG_VERSION"))
                .font_weight(FontWeightKeyword::Regular)
                .font_size(15.0)
                .top(Stretch(1.0));
        }).height(Stretch(0.2))
        .bottom(Pixels(5.0))
        .class("background-banner");
        // Sliders
        HStack::new(cx, |cx|{
            VStack::new(cx, |cx| {
                Label::new(cx, "Flanger");
                ParamSlider::new(cx, Data::params, |params| &params.flanging);
                Label::new(cx, "Dry Delay (Chaos)");
                ParamSlider::new(cx, Data::params, |params| &params.chaos);
                Label::new(cx, "Phaser");
                ParamSlider::new(cx, Data::params, |params| &params.phasing);
            })
            .left(Stretch(1.0))
            .right(Stretch(1.0));
            VStack::new(cx, |cx| {
                Label::new(cx, "Speed");
                ParamSlider::new(cx, Data::params, |params| &params.speed);
                Label::new(cx, "Dry/Wet");
                ParamSlider::new(cx, Data::params, |params| &params.strength);
            })
            .left(Stretch(1.0))
            .right(Stretch(1.0));
        }).class("background-main");
    })
}
