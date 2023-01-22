//! Editor module using vizia.

use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::config::{ACCENT_TRACK, NAME, TRACKS};
use crate::AppParams;

/// Size of the grid cells.
const GRID_CELL_SIZE: Units = Units::Pixels(25.0);

/// Spacing of the grid cells.
const GRID_CELL_SPACING: Units = Pixels(3.0);

/// Row height of the grid, must be `GRID_CELL_SIZE` + 2 * `GRID_CELL_SPACING`.
const GRID_ROW_HEIGHT: Units = Units::Pixels(31.0);

/// Width of additional spacer after columns.
const GRID_COL_SPACER_WIDTH: Units = Pixels(3.0);

/// Height of additional spacer between rows.
const GRID_ROW_SPACER_HEIGHT: Units = Pixels(3.0);

/// Width of spacer between various elements.
const ELEMENT_SPACER_WIDTH: Units = Pixels(10.0);

#[derive(Debug)]
enum AppEvent {
    CellClick(usize, usize),
}

#[derive(Lens)]
struct Data {
    params: Arc<AppParams>,
}

impl Model for Data {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::CellClick(track, step) => {
                let param = &self.params.pattern.steps[*track][*step];
                param.store(!param.load(Ordering::Relaxed), Ordering::Relaxed);
                self.params.pattern_changed.store(true, Ordering::Relaxed);
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(800, 500)
}

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
                grid(cx);

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
                    Element::new(cx).top(GRID_ROW_SPACER_HEIGHT);
                    HStack::new(cx, |cx| {
                        param_slider(cx, Data::params, |params| &params.accent_velocity);
                    })
                    .height(GRID_ROW_HEIGHT);
                });
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

/// Create the grid.
fn grid(cx: &mut Context) {
    VStack::new(cx, move |cx| {
        Binding::new(
            cx,
            Data::params.map(move |params| params.active_step.load(Ordering::Relaxed)),
            move |cx, param| {
                let active_step = param.get(cx);
                for track in 0..TRACKS {
                    if track == TRACKS - 1 {
                        // Add some space before the accent track.
                        Element::new(cx).top(GRID_ROW_SPACER_HEIGHT);
                    }
                    VStack::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            let label = if track == ACCENT_TRACK as usize {
                                "Acc".to_owned()
                            } else {
                                format!("{}", track + 1)
                            };
                            Label::new(cx, label.as_str())
                                .width(Pixels(30.0))
                                .space(Stretch(0.5));
                            for step in 0..16 {
                                Binding::new(
                                    cx,
                                    Data::params.map(move |params| {
                                        params.pattern.steps[track][step].load(Ordering::Relaxed)
                                    }),
                                    move |cx, param| {
                                        let cell_state = param.get(cx);
                                        let mut cell = VStack::new(cx, |cx| {
                                            Element::new(cx).class("content");
                                        })
                                        .size(GRID_CELL_SIZE)
                                        .space(GRID_CELL_SPACING)
                                        .child_space(Pixels(3.0))
                                        .class("step")
                                        .on_press_down(move |eh| {
                                            eh.emit(AppEvent::CellClick(track, step));
                                        });
                                        if step == active_step as usize {
                                            cell = cell.class("current");
                                        }
                                        if cell_state {
                                            cell.class("active");
                                        }
                                        if step % 4 == 3 && step != 15 {
                                            // Add addtional space after 4 cells.
                                            Element::new(cx).right(GRID_COL_SPACER_WIDTH);
                                        }
                                    },
                                );
                            }
                        });
                    });
                }
            },
        );
    })
    .id("grid");
}

/// Create a parameter slider.
fn param_slider<L, Params, P, FMap>(cx: &mut Context, params: L, params_to_param: FMap)
where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: nih_plug::prelude::Param + 'static,
    FMap: Fn(&Params) -> &P + Copy + 'static,
{
    ParamSlider::new(cx, params, params_to_param)
        .height(Pixels(20.0))
        .width(Pixels(70.0))
        .top(Stretch(0.5))
        .bottom(Stretch(0.5))
        .class("slider");
}

/// Create a parameter button.
fn param_button<L, Params, P, FMap>(cx: &mut Context, params: L, params_to_param: FMap)
where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: nih_plug::prelude::Param + 'static,
    FMap: Fn(&Params) -> &P + Copy + 'static,
{
    ParamButton::new(cx, params, params_to_param)
        .height(Pixels(20.0))
        .width(Pixels(20.0))
        .top(Stretch(0.5))
        .bottom(Stretch(0.5))
        .font_size(1.0)
        .class("button");
}
