use crate::timeseries::Time;
use crate::{polar::PolarVec3, state, timeseries, RenderMode};
use bevy::ecs::system::Res;
use bevy::{
    ecs::{component::Component, system::Query},
    gizmos::gizmos::Gizmos,
    math::{Quat, Vec3},
    render::color::Color,
};

#[derive(Clone, Debug, Default, Component)]
pub struct State {
    pub pos: Vec3,
    pub vel: Vec3,
}

impl State {
    pub fn with_xyz(mut self, x: f32, y: f32, z: f32) -> Self {
        self.pos = Vec3::new(x, y, z);
        self
    }

    pub fn with_vel(mut self, x: f32, y: f32, z: f32) -> Self {
        self.vel = Vec3::new(x, y, z);
        self
    }
}

pub fn render_states(
    mode: Res<RenderMode>,
    truth_query: Query<(&state::State, &timeseries::Active)>,
    mut gizmos: Gizmos,
) {
    let color = Color::BLACK;
    for (state, active) in truth_query.iter() {
        if !active.0 {
            continue;
        }
        match mode.as_ref() {
            RenderMode::Cartesian => {
                gizmos.sphere(state.pos, Quat::default(), 1000.0, color);
            }
            RenderMode::Spherical => {
                let polar: PolarVec3 = state.pos.into();
                gizmos.sphere(polar.direct_vec3(), Quat::default(), 0.006, color);
            }
        }
    }
}

pub fn render_history(
    time: Res<Time>,
    mode: Res<RenderMode>,
    truth_query: Query<(&timeseries::TimeSeries<State>, &timeseries::Active)>,
    mut gizmos: Gizmos,
) {
    let color = Color::BLACK;
    for (series, active) in truth_query.iter() {
        if !active.0 {
            continue;
        }
        match mode.as_ref() {
            RenderMode::Cartesian => {
                gizmos.linestrip(series.before(time.0).map(|state| state.pos), color)
                //gizmos.sphere(state.pos, Quat::default(), 1000.0, color);
            }
            RenderMode::Spherical => gizmos.linestrip(
                series
                    .before(time.0)
                    .map(|state| PolarVec3::from(state.pos).direct_vec3()),
                color,
            ),
        }
    }
}
