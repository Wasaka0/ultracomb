use nih_plug::prelude::{Editor};
use nih_plug_vizia::vizia::{prelude::*};
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
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
        assets::register_noto_sans_regular(cx);
        assets::register_noto_sans_bold(cx);
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
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Bold)
                .font_size(25.0)
                .child_left(Pixels(7.0));
            Label::new(cx, env!("CARGO_PKG_VERSION"))
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Regular)
                .font_size(15.0)
                .top(Stretch(1.0));
        }).height(Stretch(0.2))
        .col_between(Pixels(5.0))
        .child_bottom(Pixels(5.0))
        .class("background-banner");
        // Sliders
        HStack::new(cx, |cx|{
            VStack::new(cx, |cx| {
                Label::new(cx, "Flanger");
                ParamSlider::new(cx, Data::params, |params| &params.flanging);
                Label::new(cx, "Chaos");
                ParamSlider::new(cx, Data::params, |params| &params.chaos);
                Label::new(cx, "Phaser");
                ParamSlider::new(cx, Data::params, |params| &params.phasing);
            })
            .row_between(Pixels(1.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
            VStack::new(cx, |cx| {
                Label::new(cx, "Speed");
                ParamSlider::new(cx, Data::params, |params| &params.speed);
                Label::new(cx, "Dry/Wet");
                ParamSlider::new(cx, Data::params, |params| &params.strength);
            })
            .row_between(Pixels(1.0))
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0));
        }).class("background-main");
        ResizeHandle::new(cx);
    })
}
