//! VST3 plugin version of dr-seq.

use nih_plug::prelude::*;

use dr_seq_core::App;

nih_export_vst3!(App);
