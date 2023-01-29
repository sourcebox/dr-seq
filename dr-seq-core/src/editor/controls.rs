//! Common control elements with some individual styling.

use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;

/// Creates a parameter slider.
pub fn param_slider<L, Params, P, FMap>(cx: &mut Context, params: L, params_to_param: FMap)
where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: nih_plug::prelude::Param + 'static,
    FMap: Fn(&Params) -> &P + Copy + 'static,
{
    ParamSlider::new(cx, params, params_to_param)
        .height(Pixels(20.0))
        .width(Pixels(70.0))
        .top(Stretch(0.5))
        .bottom(Stretch(0.5));
}

/// Creates a parameter button.
pub fn param_button<L, Params, P, FMap>(cx: &mut Context, params: L, params_to_param: FMap)
where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: nih_plug::prelude::Param + 'static,
    FMap: Fn(&Params) -> &P + Copy + 'static,
{
    ParamButton::new(cx, params, params_to_param)
        .height(Pixels(20.0))
        .width(Pixels(20.0))
        .top(Stretch(0.5))
        .bottom(Stretch(0.5))
        .font_size(1.0);
}
