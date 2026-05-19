//! VST3 plugin version of dr-seq.

use nice_plug::prelude::*;

use dr_seq_core::App;

nice_export_vst3!(App);
