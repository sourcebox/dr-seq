use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::DrSeqParams;

#[derive(Debug)]
enum AppEvent {
    CellClick(usize, usize),
}

#[derive(Lens)]
struct Data {
    params: Arc<DrSeqParams>,
}

impl Model for Data {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::CellClick(track, step) => {
                let param = &self.params.pattern.steps[*track][*step];
                param.store(!param.load(Ordering::Relaxed), Ordering::Relaxed);
            }
        });
    }
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(600, 400)
}

pub(crate) fn create(
    params: Arc<DrSeqParams>,
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
                    assets::NOTO_SANS_THIN,
                ))])
                .font_size(20.0);
        })
        .child_space(Pixels(5.0))
        .height(Pixels(40.0))
        .id("header");

        let active_step = 2;

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
                                .on_press(move |eh| {
                                    eh.emit(AppEvent::CellClick(track, step));
                                });
                                if step == active_step {
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
    })
}
