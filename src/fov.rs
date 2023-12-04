use std::f32::consts::FRAC_PI_2;

use bevy::{
    ecs::system::Res,
    ecs::system::Resource,
    gizmos::gizmos::Gizmos,
    math::{Quat, Vec2, Vec3},
    render::color::Color,
};

use crate::{polar::PolarVec3, RenderMode};

#[derive(Resource)]
pub struct FoV {
    range: f32,
    az: f32,
    el: f32,
}

impl Default for FoV {
    fn default() -> Self {
        Self {
            range: 200_000.0,
            az: FRAC_PI_2,
            el: FRAC_PI_2,
        }
    }
}

#[derive(Resource)]
pub struct MaxRange(f32);

pub fn render_fov(mode: Res<RenderMode>, fov: Res<FoV>, mut gizmos: Gizmos) {
    let color = Color::GRAY;
    match mode.as_ref() {
        RenderMode::Spherical => {
            let size = Vec2::new(fov.az, fov.el);
            gizmos.rect(Vec3::Z * fov.range, Quat::default(), size, color);
        }
        RenderMode::Cartesian => {
            let tl = PolarVec3::new(fov.range, -fov.az / 2.0, fov.el / 2.0);
            let tr = PolarVec3::new(fov.range, fov.az / 2.0, fov.el / 2.0);
            let bl = PolarVec3::new(fov.range, -fov.az / 2.0, -fov.el / 2.0);
            let br = PolarVec3::new(fov.range, fov.az / 2.0, -fov.el / 2.0);
            gizmos.line(Vec3::default(), tl.into(), color);
            gizmos.line(Vec3::default(), tr.into(), color);
            gizmos.line(Vec3::default(), bl.into(), color);
            gizmos.line(Vec3::default(), br.into(), color);

            let top = PolarVec3::new(fov.range, 0.0, fov.el / 2.0);
            let bottom = PolarVec3::new(fov.range, 0.0, -fov.el / 2.0);
            gizmos.line(Vec3::default(), top.into(), color);
            gizmos.line(Vec3::default(), bottom.into(), color);

            let steps = 5;

            // The back two arcs
            let positions: Vec<_> = (0..steps + 1)
                .map(|i| {
                    let az = (fov.az / (steps as f32)) * (i as f32) - (fov.az / 2.0);
                    let p = PolarVec3::new(fov.range, az, 0.0);
                    p.into()
                })
                .collect();
            gizmos.linestrip(positions, color);

            let positions: Vec<_> = (0..steps + 1)
                .map(|i| {
                    let el = (fov.el / (steps as f32)) * (i as f32) - (fov.el / 2.0);
                    dbg!(el);
                    let p = PolarVec3::new(fov.range, 0.0, el);
                    p.into()
                })
                .collect();
            gizmos.linestrip(positions, color);

            // Top and bottom arc
            let positions: Vec<_> = (0..steps + 1)
                .map(|i| {
                    let az = (fov.az / (steps as f32)) * (i as f32) - (fov.az / 2.0);
                    let p = PolarVec3::new(fov.range, az, fov.el / 2.0);
                    p.into()
                })
                .collect();
            gizmos.linestrip(positions, color);

            let positions: Vec<_> = (0..steps + 1)
                .map(|i| {
                    let az = (fov.az / (steps as f32)) * (i as f32) - (fov.az / 2.0);
                    let p = PolarVec3::new(fov.range, az, -fov.el / 2.0);
                    p.into()
                })
                .collect();
            gizmos.linestrip(positions, color);

            // Left and right arc
            let positions: Vec<_> = (0..steps + 1)
                .map(|i| {
                    let el = (fov.el / (steps as f32)) * (i as f32) - (fov.el / 2.0);
                    let p = PolarVec3::new(fov.range, fov.az / 2.0, el);
                    p.into()
                })
                .collect();
            gizmos.linestrip(positions, color);

            let positions: Vec<_> = (0..steps + 1)
                .map(|i| {
                    let el = (fov.el / (steps as f32)) * (i as f32) - (fov.el / 2.0);
                    let p = PolarVec3::new(fov.range, -fov.az / 2.0, el);
                    p.into()
                })
                .collect();
            gizmos.linestrip(positions, color);
        }
    }
}
