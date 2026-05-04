//! Tracks with cells for each step.

use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::mpsc;

use vizia_plug::vizia::prelude::*;

use super::EditorEvent;
use super::controls::*;
use super::style::*;
use crate::AppParams;
use crate::config::*;
use crate::params::StepState;

/// Creates the tracks.
pub fn create(cx: &mut Context, params: Arc<AppParams>) {
    VStack::new(cx, move |cx| {
        for track in 0..TRACKS {
            if track == TRACKS - 1 {
                // Add some space before the accent track.
                Element::new(cx).height(TRACK_ROW_SPACER_HEIGHT);
                Element::new(cx).height(TRACK_ROW_SPACER_HEIGHT);
            }

            create_track(
                cx,
                params.clone(),
                track,
                params.current_step.load(Ordering::Relaxed),
            );
        }
    })
    .id("tracks");
}

/// Creates a single track.
fn create_track(cx: &mut Context, params: Arc<AppParams>, track: usize, current_step: usize) {
    let enable_params = [
        &params.track1_enable,
        &params.track2_enable,
        &params.track3_enable,
        &params.track4_enable,
        &params.track5_enable,
        &params.track6_enable,
        &params.track7_enable,
        &params.track8_enable,
    ];

    let delay_params = [
        &params.track1_delay,
        &params.track2_delay,
        &params.track3_delay,
        &params.track4_delay,
        &params.track5_delay,
        &params.track6_delay,
        &params.track7_delay,
        &params.track8_delay,
    ];

    let accent_track = track == TRACKS - 1;

    VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, TRACK_LABELS[track]).width(Pixels(45.0));

            let event_sender = params.editor_event_sender.lock().unwrap().clone();

            for step in 0..16 {
                let signal = SyncSignal::new(Arc::new(AtomicU32::new(
                    params.pattern.steps[track][step].load(Ordering::Relaxed),
                )));
                StepCell::new(cx, signal, track, step, accent_track, event_sender.clone());

                if step % 4 == 3 && step != 15 {
                    // Add addtional space after block of 4 cells.
                    Element::new(cx).width(GRID_COL_SPACER_WIDTH);
                }
            }

            if !accent_track {
                HStack::new(cx, |cx| {
                    param_button(cx, enable_params[track]);
                    Element::new(cx).width(ELEMENT_SPACER_WIDTH);
                    param_slider(cx, delay_params[track]);
                });
            }
        });
    });
}

/// Cell for a single step in the beat grid.
struct StepCell;

impl View for StepCell {}

impl StepCell {
    /// Returns a new cell.
    fn new(
        cx: &mut Context,
        state: SyncSignal<Arc<AtomicU32>>,
        track: usize,
        step: usize,
        accent_step: bool,
        event_sender: mpsc::SyncSender<EditorEvent>,
    ) -> Handle<'_, Self> {
        Self.build(cx, move |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx).class("content");
            })
            .class("step")
            .bind(state, move |handle| {
                let step_state = StepState::from(state.get().load(Ordering::Relaxed));
                handle
                    .toggle_class("normal", step_state == StepState::Normal)
                    .toggle_class("accent", step_state == StepState::Accent)
                    .toggle_class("weak", step_state == StepState::Weak)
                    .toggle_class("ghost", step_state == StepState::Ghost);
            });
        })
        .on_mouse_down(move |eh, _| {
            let shift = eh.modifiers().contains(Modifiers::SHIFT);
            let alt = eh.modifiers().contains(Modifiers::ALT);

            let step_state = StepState::from(state.get().load(Ordering::Relaxed));

            let mut new_state = match (shift, alt) {
                (true, false) => StepState::Weak,
                (true, true) => StepState::Ghost,
                (false, true) => StepState::Accent,
                _ => StepState::Normal,
            };

            if accent_step && new_state != StepState::Off {
                // Accent track has only on/off steps, so the on
                // state is always `Accent`.
                new_state = StepState::Accent;
            }

            if new_state == step_state {
                new_state = StepState::Off;
            }

            state.update(|s| s.store(new_state.into(), Ordering::Relaxed));

            // Send an event back to the engine.
            event_sender
                .send(EditorEvent::CellClick(track, step, new_state))
                .ok();
        })
    }
}
