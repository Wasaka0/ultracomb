use nice_plug::prelude::{Editor};
use vizia_plug::vizia::{prelude::*};
use vizia_plug::{ create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::UltracombParams;
use crate::custom_param_slider::CustomParamSlider;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (400, 260))
}

pub(crate) fn create(
    params: Arc<UltracombParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {

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
        })
        .height(Stretch(0.2))
        .padding(Pixels(5.0))
        .class("background-banner");
        // Sliders
        HStack::new(cx, |cx|{
            VStack::new(cx, |cx| {
                Label::new(cx, "Flanger");
                CustomParamSlider::new(cx,&params.flanging).width(Stretch(1.0));
                Label::new(cx, "Dry Delay (Chaos)");
                CustomParamSlider::new(cx, &params.chaos).width(Stretch(1.0));
                Label::new(cx, "Phaser");
                CustomParamSlider::new(cx, &params.phasing).width(Stretch(1.0));
                Label::new(cx, "Speed");
                CustomParamSlider::new(cx, &params.speed).width(Stretch(1.0));
            })
            .width(Stretch(0.5))
            .padding(Pixels(5.0));
            VStack::new(cx, |cx| {
                Label::new(cx, "Low-Cut");
                CustomParamSlider::new(cx, &params.low).width(Stretch(1.0));
                Label::new(cx, "High-Cut");
                CustomParamSlider::new(cx, &params.high).width(Stretch(1.0));
                Label::new(cx, "Dry/Wet");
                CustomParamSlider::new(cx, &params.strength).width(Stretch(1.0));
                Label::new(cx, "Multiplier");
                CustomParamSlider::new(cx, &params.multiplier).width(Stretch(1.0));
            })
            .width(Stretch(0.5))
            .padding(Pixels(7.0));  
        })
        .class("background-main")
        .padding(Pixels(2.0));
    })
}