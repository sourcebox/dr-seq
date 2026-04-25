//! Editor module using vizia.

mod controls;
mod grid;
mod style;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use vizia_plug::vizia::prelude::*;
use vizia_plug::widgets::*;
use vizia_plug::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::config::NAME;
use crate::params::StepState;
use crate::AppParams;
use controls::*;
use style::*;

#[derive(Debug, Clone)]
pub enum EditorEvent {
    /// Click on a cell with track, bar and step.
    CellClick(usize, usize, usize, StepState),
}

/// Returns the default state.
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 500))
}

/// Create the editor.
pub(crate) fn create(
    params: Arc<AppParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        // TODO: check
        // assets::register_noto_sans_light(cx);
        // assets::register_noto_sans_thin(cx);
        // assets::register_noto_sans_bold(cx);

        cx.add_stylesheet(include_str!("editor/theme.css")).ok();

        ResizeHandle::new(cx);

        HStack::new(cx, |cx| {
            Label::new(cx, NAME).font_size(20.0);
        })
        .padding(Pixels(5.0))
        .height(Pixels(40.0))
        .id("header");

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                grid::create(cx, &params);

                Element::new(cx).width(Pixels(10.0));

                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track1_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track1_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track2_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track2_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track3_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track3_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track4_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track4_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track5_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track5_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track6_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track6_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track7_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track7_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, &params.track8_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, &params.track8_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                });

                Element::new(cx).width(Pixels(20.0));

                VStack::new(cx, |cx| {
                    Label::new(cx, "Velocity");
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Default");
                    param_slider(cx, &params.default_velocity);
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
                })
                .top(Pixels(5.0))
                .height(Pixels(250.0));
            })
            .height(Pixels(300.0));

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Swing").top(Pixels(5.0)).right(Pixels(10.0));
                    ParamSlider::new(cx, &params.swing).class("slider");
                });
            })
            .top(Pixels(20.0));
        })
        .space(Pixels(10.0));
    })
}
