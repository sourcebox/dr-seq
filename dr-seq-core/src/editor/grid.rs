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
        for track in 0..TRACKS {
            if track == TRACKS - 1 {
                // Add some space before the accent track.
                Element::new(cx).top(GRID_ROW_SPACER_HEIGHT);
            }

            create_track(
                cx,
                track,
                bar,
                Data::params.map(move |params| params.current_step.load(Ordering::Relaxed)),
            );
        }
    })
    .width(Pixels(550.0))
    .height(Pixels(310.0))
    .id("grid");
}

/// Creates a single track.
fn create_track(cx: &mut Context, track: usize, bar: usize, current_step: impl Lens<Target = i32>) {
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
fn create_step(
    cx: &mut Context,
    track: usize,
    bar: usize,
    step: usize,
    current_step: impl Lens<Target = i32>,
) {
    let state_lens = Data::params
        .map(move |params| params.pattern.steps[track][bar][step].load(Ordering::Relaxed));

    VStack::new(cx, |cx| {
        Element::new(cx).class("content");
    })
    .size(GRID_CELL_SIZE)
    .space(GRID_CELL_SPACING)
    .child_space(state_lens.map(|state| {
        if StepState::from(*state) == StepState::Weak {
            Pixels(6.0)
        } else {
            Pixels(3.0)
        }
    }))
    .class("step")
    .toggle_class(
        "current",
        current_step.map(move |current| *current as usize == step),
    )
    .toggle_class(
        "default",
        state_lens.map(|state| {
            let state = StepState::from(*state);
            state == StepState::Default
        }),
    )
    .toggle_class(
        "weak",
        state_lens.map(|state| {
            let state = StepState::from(*state);
            state == StepState::Weak
        }),
    )
    .toggle_class(
        "strong",
        state_lens.map(|state| {
            let state = StepState::from(*state);
            state == StepState::Strong
        }),
    )
    .on_press_down(move |eh| {
        let state_lens = Data::params
            .map(move |params| params.pattern.steps[track][bar][step].load(Ordering::Relaxed));
        if let Some(state) = state_lens.get_fallible(eh) {
            let state = StepState::from(state);

            let shift = eh.modifiers().contains(Modifiers::SHIFT);
            let alt = eh.modifiers().contains(Modifiers::ALT);

            let mut new_state = match state {
                StepState::Off => {
                    if shift {
                        StepState::Weak
                    } else if alt {
                        StepState::Strong
                    } else {
                        StepState::Default
                    }
                }
                StepState::Default => {
                    if shift {
                        StepState::Weak
                    } else if alt {
                        StepState::Strong
                    } else {
                        StepState::Off
                    }
                }
                _ => StepState::Off,
            };

            if track == TRACKS - 1 && new_state != StepState::Off {
                // Accent track has only on/off steps, so the on
                // state is always `Strong`.
                new_state = StepState::Strong;
            }

            eh.emit(EditorEvent::CellClick(track, bar, step, new_state));
        }
    });

    if step % 4 == 3 && step != 15 {
        // Add addtional space after block of 4 cells.
        Element::new(cx).right(GRID_COL_SPACER_WIDTH);
    }
}
