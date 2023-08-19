use std::time::Duration;
use bevy::core_pipeline::clear_color::ClearColorConfig::Custom;
use bevy::math::Vec4;
use bevy::prelude::{Camera2d, Color};
use bevy_tweening::{Animator, EaseFunction, Lens, RepeatStrategy, Tween};

pub struct Camera2dLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<Camera2d> for Camera2dLens {
    fn lerp(&mut self, target: &mut Camera2d, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.clear_color = Custom(value.into());
    }
}

pub fn blink_background(duration: Duration, start: Color, end: Color) -> Animator<Camera2d> {
    let tween = Tween::new(
        EaseFunction::CubicOut,
        duration,
        Camera2dLens { start, end, },
    ).with_repeat_count(2)
        .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    Animator::new(tween)
}