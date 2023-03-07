//! Editor module using vizia.

mod controls;
mod grid;
mod style;

use std::sync::Arc;

use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

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

#[derive(Lens)]
struct Data {
    params: Arc<AppParams>,
}

impl Model for Data {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        if let Ok(sender) = self.params.editor_event_sender.lock() {
            event.map(|app_event: &EditorEvent, _| {
                sender.send(app_event.clone()).ok();
            });
        };
    }
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
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);
        assets::register_noto_sans_bold(cx);

        cx.add_theme(include_str!("editor/theme.css"));

        Data {
            params: params.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        HStack::new(cx, |cx| {
            Label::new(cx, NAME)
                .font_family(vec![FamilyOwned::Name(String::from(
                    assets::NOTO_SANS_LIGHT,
                ))])
                .font_size(20.0);
        })
        .child_space(Pixels(5.0))
        .height(Pixels(40.0))
        .id("header");

        VStack::new(cx, move |cx| {
            HStack::new(cx, move |cx| {
                grid::create(cx);

                Element::new(cx).width(Pixels(10.0));

                VStack::new(cx, move |cx| {
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track1_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track1_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track2_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track2_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track3_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track3_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track4_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track4_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track5_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track5_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track6_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track6_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track7_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track7_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_button(cx, Data::params, |params| &params.track8_enable);
                        Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                        param_slider(cx, Data::params, |params| &params.track8_delay);
                    })
                    .height(GRID_ROW_HEIGHT);
                });

                Element::new(cx).width(Pixels(20.0));

                VStack::new(cx, |cx| {
                    Label::new(cx, "Velocity");
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Default");
                    param_slider(cx, Data::params, |params| &params.default_velocity);
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Accent");
                    param_slider(cx, Data::params, |params| &params.accent_velocity);
                    ParamButton::new(cx, Data::params, |params| &params.accent_vel_mode)
                        .height(Pixels(20.0))
                        .top(Pixels(3.0))
                        .with_label("abs");
                    Element::new(cx).height(Pixels(10.0));
                    Label::new(cx, "Weak");
                    param_slider(cx, Data::params, |params| &params.weak_velocity);
                })
                .top(Pixels(5.0))
                .height(Pixels(100.0));
            });

            VStack::new(cx, move |cx| {
                HStack::new(cx, move |cx| {
                    Label::new(cx, "Swing").top(Pixels(5.0)).right(Pixels(10.0));
                    ParamSlider::new(cx, Data::params, |params| &params.swing).class("slider");
                });
            })
            .top(Pixels(20.0));
        })
        .space(Pixels(10.0));
    })
}
