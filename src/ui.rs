use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res, ResMut},
    },
    input::keyboard::KeyCode,
    input::Input,
    text::Text,
};

use crate::timeseries::{Time, TimeFlow};

#[derive(Component)]
pub struct TimeControlText;

pub fn time_control(
    keycode: Res<Input<KeyCode>>,
    mut time: ResMut<Time>,
    mut flow: ResMut<TimeFlow>,
    mut query: Query<&mut Text, With<TimeControlText>>,
) {
    if keycode.just_pressed(KeyCode::R) {
        time.0 = 0.0;
    }

    if keycode.just_pressed(KeyCode::Comma) {
        let delta = if flow.delta.abs() <= 0.01 {
            0.002
        } else {
            0.01
        };
        flow.delta -= delta;
    }

    if keycode.just_pressed(KeyCode::Period) {
        let delta = if flow.delta.abs() <= 0.01 {
            0.002
        } else {
            0.01
        };
        flow.delta += delta;
    }

    if keycode.just_pressed(KeyCode::Left) {
        time.0 -= flow.delta;
    }

    if keycode.just_pressed(KeyCode::Right) {
        time.0 += flow.delta;
    }

    if keycode.just_pressed(KeyCode::Space) {
        flow.paused = !flow.paused;
    }
    let mut text = query.single_mut();
    text.sections[1].value = format!("{:?}", flow);
}
