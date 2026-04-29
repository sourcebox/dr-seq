//! Grid with cells for each step.

use std::sync::Arc;
use std::sync::atomic::Ordering;

use vizia_plug::vizia::prelude::*;

use super::EditorEvent;
use super::style::*;
use crate::AppParams;
use crate::config::*;
use crate::params::StepState;

/// Creates the grid.
pub fn create(cx: &mut Context, params: &Arc<AppParams>) {
    // TODO: get real bar number
    let bar = 0;

    VStack::new(cx, move |cx| {
        for track in 0..TRACKS {
            if track == TRACKS - 1 {
                // Add some space before the accent track.
                Element::new(cx).height(GRID_ROW_SPACER_HEIGHT);
                Element::new(cx).height(GRID_ROW_SPACER_HEIGHT);
            }

            create_track(
                cx,
                params,
                track,
                bar,
                params.current_step.load(Ordering::Relaxed),
            );
        }
    })
    .id("grid");
}

/// Creates a single track.
fn create_track(
    cx: &mut Context,
    params: &Arc<AppParams>,
    track: usize,
    bar: usize,
    current_step: i32,
) {
    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, TRACK_LABELS[track]).width(Pixels(45.0));

            for step in 0..16 {
                create_step(cx, params, track, bar, step, current_step);
            }
        });
    });
}

/// Creates a single step.
fn create_step(
    cx: &mut Context,
    params: &Arc<AppParams>,
    track: usize,
    bar: usize,
    step: usize,
    current_step: i32,
) {
    let step_state =
        StepState::from(params.pattern.steps[track][bar][step].load(Ordering::Relaxed));

    VStack::new(cx, |cx| {
        Element::new(cx).class("content");
    })
    .size(GRID_CELL_SIZE)
    .space(GRID_CELL_SPACING)
    .padding(if step_state == StepState::Weak {
        Pixels(6.0)
    } else {
        Pixels(3.0)
    })
    .class("step")
    .toggle_class("current", current_step as usize == step)
    .toggle_class("default", step_state == StepState::Default)
    .toggle_class("weak", step_state == StepState::Weak)
    .toggle_class("strong", step_state == StepState::Strong)
    .on_press_down(move |eh| {
        let shift = eh.modifiers().contains(Modifiers::SHIFT);
        let alt = eh.modifiers().contains(Modifiers::ALT);

        let mut new_state = match step_state {
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
    });

    if step % 4 == 3 && step != 15 {
        // Add addtional space after block of 4 cells.
        Element::new(cx).width(GRID_COL_SPACER_WIDTH);
    }
}
