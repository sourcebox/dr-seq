//! Common control elements with some individual styling.

use nih_plug::prelude::Param;
use vizia_plug::vizia::prelude::*;
use vizia_plug::widgets::*;

/// Creates a parameter slider.
pub fn param_slider<'c, 'p, P>(cx: &'c mut Context, param: &'p P)
where
    'p: 'c,
    P: Param + 'static,
{
    ParamSlider::new(cx, param)
        .height(Pixels(20.0))
        .width(Pixels(70.0))
        .top(Stretch(0.5))
        .bottom(Stretch(0.5));
}

/// Creates a parameter button.
pub fn param_button<'c, 'p, P>(cx: &'c mut Context, param: &'p P)
where
    'p: 'c,
    P: Param + 'static,
{
    ParamButton::new(cx, param)
        .with_label("")
        .height(Pixels(20.0))
        .width(Pixels(20.0))
        .top(Stretch(0.5))
        .bottom(Stretch(0.5));
}
