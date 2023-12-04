use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Query, Res};
use bevy::gizmos::gizmos::Gizmos;
use bevy::math::{Quat, Vec3};
use bevy::render::color::Color;

use crate::polar::PolarVec3;
use crate::timeseries::{Active, TimeSeries};
use crate::RenderMode;

#[derive(Debug, Clone, Component)]
pub struct BeamState {
    pub width: f32,
    pub target: PolarVec3,
    pub index: usize,
}

pub fn render_beams(
    mode: Res<RenderMode>,
    beam_query: Query<(&BeamState, &Active)>,
    mut gizmos: Gizmos,
) {
    const COLORS: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::ORANGE];

    for (beam, active) in beam_query.iter() {
        if !active.0 {
            continue;
        }

        let color = COLORS[beam.index];
        match mode.as_ref() {
            RenderMode::Spherical => {
                //gizmos.circle(beam.target.direct_vec3(), Vec3::NEG_X, beam.width / 2.0, color);
                gizmos.sphere(
                    beam.target.direct_vec3(),
                    Quat::default(),
                    beam.width / 2.0,
                    color,
                );
            }
            RenderMode::Cartesian => {
                // Cartesian beams are drawn as a circle at the target range, and a line back to
                // the origin from each corner to make a cone.
                let beam_center = beam.target.clone().into();
                let beam_radius = beam.target.range * (beam.width / 2.0).sin();
                gizmos.circle(beam_center, -Vec3::Z, beam_radius, color);
                gizmos.line(Vec3::default(), beam_center + beam_radius * Vec3::X, color);
                gizmos.line(Vec3::default(), beam_center - beam_radius * Vec3::X, color);
                gizmos.line(Vec3::default(), beam_center + beam_radius * Vec3::Y, color);
                gizmos.line(Vec3::default(), beam_center - beam_radius * Vec3::Y, color);
            }
        }
    }
}

#[derive(Bundle)]
pub struct BeamBundle {
    pub state: BeamState,
    pub history: TimeSeries<BeamState>,
    pub active: Active,
}
