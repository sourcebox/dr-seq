//! Editor module using vizia.

use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::AppParams;

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
    ViziaState::from_size(800, 400)
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
            Label::new(cx, "Dr. Seq")
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
                VStack::new(cx, move |cx| {
                    grid(cx);
                });

                VStack::new(cx, move |cx| {
                    delay_slider(cx, Data::params, |params| &params.track1_delay);
                    delay_slider(cx, Data::params, |params| &params.track2_delay);
                    delay_slider(cx, Data::params, |params| &params.track3_delay);
                    delay_slider(cx, Data::params, |params| &params.track4_delay);
                    delay_slider(cx, Data::params, |params| &params.track5_delay);
                    delay_slider(cx, Data::params, |params| &params.track6_delay);
                    delay_slider(cx, Data::params, |params| &params.track7_delay);
                    delay_slider(cx, Data::params, |params| &params.track8_delay);
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
    Binding::new(
        cx,
        Data::params.map(move |params| params.active_step.load(Ordering::Relaxed)),
        move |cx, param| {
            let active_step = param.get(cx);
            for track in 0..8 {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
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
                                    .size(Pixels(30.0))
                                    .space(Pixels(2.0))
                                    .child_space(Pixels(3.0))
                                    .class("step")
                                    .on_press_down(move |eh| {
                                        eh.emit(AppEvent::CellClick(track, step));
                                    });
                                    if step == active_step as usize {
                                        cell = cell.class("current");
                                    }
                                    if cell_state {
                                        cell = cell.class("active");
                                    }
                                    if step % 4 == 3 {
                                        cell.right(Pixels(4.0));
                                    }
                                },
                            );
                        }
                    });
                })
                .height(Pixels(30.0));
            }
        },
    );
}

/// Create a delay slider.
fn delay_slider<L, Params, P, FMap>(cx: &mut Context, params: L, params_to_param: FMap)
where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: nih_plug::prelude::Param + 'static,
    FMap: Fn(&Params) -> &P + Copy + 'static,
{
    VStack::new(cx, |cx| {
        ParamSlider::new(cx, params, params_to_param)
            .height(Pixels(20.0))
            .width(Pixels(100.0))
            .top(Pixels(8.0))
            .class("slider");
    })
    .height(Pixels(34.0));
}
