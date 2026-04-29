//! Editor module using vizia.

mod controls;
mod style;
mod tracks;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use vizia_plug::vizia::prelude::*;
use vizia_plug::widgets::*;
use vizia_plug::{ViziaState, ViziaTheming, create_vizia_editor};

use crate::AppParams;
use crate::config::NAME;
use crate::params::StepState;
use controls::*;

#[derive(Debug, Clone)]
pub enum EditorEvent {
    /// Click on a cell with track, bar and step.
    CellClick(usize, usize, usize, StepState),
}

/// Returns the default state.
pub fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 500))
}

/// Create the editor.
pub fn create(params: Arc<AppParams>, editor_state: Arc<ViziaState>) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        // TODO: check
        // assets::register_noto_sans_light(cx);
        // assets::register_noto_sans_thin(cx);
        // assets::register_noto_sans_bold(cx);

        cx.add_stylesheet(include_str!("theme.css")).ok();

        ResizeHandle::new(cx);

        HStack::new(cx, |cx| {
            Label::new(cx, NAME);
        })
        .id("header");

        Grid::new(
            cx,
            vec![Pixels(650.0), Pixels(120.0)],
            vec![Pixels(310.0)],
            |cx| {
                VStack::new(cx, |cx| {
                    tracks::create(cx, params.clone());
                })
                .row_start(0)
                .column_start(0);

                VStack::new(cx, |cx| {
                    Label::new(cx, "Velocity");
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Normal");
                    param_slider(cx, &params.normal_velocity);
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Accent");
                    param_slider(cx, &params.accent_velocity);
                    ParamButton::new(cx, &params.accent_vel_mode)
                        .height(Pixels(20.0))
                        .top(Pixels(3.0))
                        .with_label("abs");
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Weak");
                    param_slider(cx, &params.weak_velocity);
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Ghost");
                    param_slider(cx, &params.ghost_velocity);
                })
                .row_start(0)
                .column_start(1);

                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Swing")
                            .padding_top(Pixels(3.0))
                            .padding_right(Pixels(10.0));
                        ParamSlider::new(cx, &params.swing).class("slider");
                    });
                })
                .row_start(1)
                .column_start(0)
                .padding_top(Pixels(10.0));
            },
        )
        .id("main");
    })
}
