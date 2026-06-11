//! Standalone version of the plugin.

use nice_plug::prelude::*;

use dr_seq_plugin::App;

fn main() {
    nice_export_standalone::<App>();
}
