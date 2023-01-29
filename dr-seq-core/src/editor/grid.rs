//! Grid with cells for each step.

use std::sync::atomic::Ordering;

use nih_plug_vizia::vizia::prelude::*;

use super::style::*;
use super::{Data, EditorEvent};
use crate::config::*;
use crate::params::StepState;

/// Creates the grid.
pub fn create(cx: &mut Context) {
    // TODO: get real bar number
    let bar = 0;

    VStack::new(cx, move |cx| {
        Binding::new(
            cx,
            Data::params.map(move |params| params.current_step.load(Ordering::Relaxed)),
            move |cx, param| {
                let current_step = param.get(cx);

                for track in 0..TRACKS {
                    if track == TRACKS - 1 {
                        // Add some space before the accent track.
                        Element::new(cx).top(GRID_ROW_SPACER_HEIGHT);
                    }

                    create_track(cx, track, bar, current_step as u32);
                }
            },
        );
    })
    .id("grid");
}

/// Creates a single track.
fn create_track(cx: &mut Context, track: usize, bar: usize, current_step: u32) {
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
                create_step(cx, track, bar, step, current_step);
            }
        });
    });
}

/// Creates a single step.
fn create_step(cx: &mut Context, track: usize, bar: usize, step: usize, current_step: u32) {
    Binding::new(
        cx,
        Data::params
            .map(move |params| params.pattern.steps[track][bar][step].load(Ordering::Relaxed)),
        move |cx, param| {
            let state = StepState::from(param.get(cx));
            let mut cell = VStack::new(cx, |cx| {
                Element::new(cx).class("content");
            })
            .size(GRID_CELL_SIZE)
            .space(GRID_CELL_SPACING)
            .child_space(Pixels(3.0))
            .class("step")
            .on_press_down(move |eh| {
                // TODO: use real shift key state when vizia
                // offers it.
                let shift = false;
                let mut new_state = match state {
                    StepState::Off => {
                        if shift {
                            StepState::Weak
                        } else {
                            StepState::Default
                        }
                    }
                    StepState::Default => {
                        if shift {
                            StepState::Weak
                        } else {
                            StepState::Off
                        }
                    }
                    _ => StepState::Off,
                };

                if track == TRACKS - 1 && new_state != StepState::Off {
                    // Accent track has only on/off steps, so the on
                    // state is always `Default`.
                    new_state = StepState::Default;
                }

                eh.emit(EditorEvent::CellClick(track, bar, step, new_state));
            });

            if step == current_step as usize {
                cell = cell.class("current");
            }

            match state {
                StepState::Default => {
                    cell.class("default");
                }
                StepState::Weak => {
                    cell.class("weak");
                }
                _ => {}
            }

            if step % 4 == 3 && step != 15 {
                // Add addtional space after block of 4 cells.
                Element::new(cx).right(GRID_COL_SPACER_WIDTH);
            }
        },
    );
}
