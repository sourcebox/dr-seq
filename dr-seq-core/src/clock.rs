//! Clock generator.

use nih_plug::buffer::Buffer;
use nih_plug::context::process::Transport;

/// Clock generator.
#[derive(Debug)]
pub struct Clock {
    /// Position in the song in pulses.
    pos_pulses: Option<f64>,

    /// Duration of a pulse in samples.
    pulse_duration_samples: Option<f64>,

    /// Buffer length in samples.
    buffer_length: usize,

    /// Iteration count.
    count: usize,

    /// Flag if transport is playing.
    playing: bool,
}

impl Clock {
    /// Returns a new instance of `Clock`.
    /// - `buffer:` Reference to the buffer object.
    /// - `transport`: Reference to the transport object.
    /// - `ppq:` Pulses per quarter note.
    pub fn new(buffer: &Buffer, transport: &Transport, ppq: f64) -> Self {
        Self {
            pos_pulses: transport.pos_beats().map(|v| v * ppq),
            pulse_duration_samples: transport
                .tempo
                .map(|v| 60.0 / (v * ppq) * transport.sample_rate as f64),
            buffer_length: buffer.samples(),
            count: 0,
            playing: transport.playing,
        }
    }
}

impl Iterator for Clock {
    /// Tuple of (pulse number, timing).
    type Item = (i32, u32);

    /// Returns the next value.
    fn next(&mut self) -> Option<Self::Item> {
        if !self.playing {
            // Transport must be playing, otherwise clock generation makes no sense.
            return None;
        }

        if let (Some(pos_pulses), Some(pulse_duration_samples)) =
            (self.pos_pulses, self.pulse_duration_samples)
        {
            // Distance to the next pulse in samples.
            let next_pulse_delta = ((pos_pulses.ceil() - pos_pulses) * pulse_duration_samples
                + self.count as f64 * pulse_duration_samples)
                .round() as u32;

            if next_pulse_delta < self.buffer_length as u32 {
                let result = Some((pos_pulses.ceil() as i32, next_pulse_delta));

                // Prepare next pulse.
                self.pos_pulses = Some(pos_pulses + 1.0);
                self.count += 1;

                return result;
            }
        }

        None
    }
}
