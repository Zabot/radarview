mod beam;
mod data;
mod fov;
mod polar;
mod state;
mod timeseries;
mod truth;
mod ui;

use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use beam::BeamState;
use data::SimulationRun;
use fov::FoV;
use timeseries::ElapsedText;
use ui::TimeControlText;

#[derive(Resource)]
pub enum RenderMode {
    Spherical,
    Cartesian,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui::time_control)
        .add_systems(Update, state::render_states)
        .add_systems(Update, beam::render_beams)
        .add_systems(Update, fov::render_fov)
        .add_systems(Update, timeseries::update_current_time::<BeamState>)
        .add_systems(Update, timeseries::update_current_time::<state::State>)
        .add_systems(Update, timeseries::advance_time)
        .add_systems(Update, timeseries::elapsed_text_update)
        .add_systems(Update, state::render_history)
        .run();
}

#[derive(Component)]
struct Ground;

//fn setup(mut commands: Commands) {
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground,
    ));

    // camera
    commands.spawn((
        Camera3dBundle {
            //projection: Projection::Perspective(PerspectiveProjection {
            //fov: FRAC_PI_2,
            //aspect_ratio: 1.0,
            //near: 0.0,
            //far: 250000.0,
            //}),
            projection: Projection::Orthographic(OrthographicProjection {
                near: -300000.0,
                far: 300000.0,
                //scale: 1.2,
                scale: 150000.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }),
            transform: Transform::from_xyz(100_000.0, 0.0, 0.0).looking_to(Vec3::Z, Vec3::Y),
            //transform: Transform::from_xyz(0.0, 0.0, 100_000.0).looking_to(Vec3::Z, Vec3::Y),
            //transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            //transform: Transform::from_xyz(75000.0, -50000.0, 0.0).looking_to(Vec3::Y, Vec3::Z),
            //transform: Transform::from_xyz(75000.0, 0.0, 0.0).looking_to(Vec3::NEG_Z, Vec3::X),
            ..default()
        },
        PanOrbitCamera {
            alpha: Some(PI),
            focus: Vec3::Z * 100_100.0,
            ..default()
        },
    ));

    //commands.insert_resource(RenderMode::Spherical);
    commands.insert_resource(RenderMode::Cartesian);
    commands.insert_resource(timeseries::Time(0.0));
    commands.insert_resource(timeseries::TimeFlow::default());
    commands.insert_resource(FoV::default());

    let sim = SimulationRun::new("./sim_3482576718.json").unwrap();
    commands.spawn_batch(sim.truths());
    commands.spawn_batch(sim.beams());

    commands.spawn((
        TextBundle::from_section(
            "Elapsed",
            TextStyle {
                color: Color::BLACK,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ElapsedText,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Space: Pause/Resume\n</>: change speed\nLeft/Right: Single step\nR: Restart\n",
                TextStyle {
                    color: Color::BLACK,
                    ..default()
                },
            ),
            TextSection::new(
                "Status",
                TextStyle {
                    color: Color::BLACK,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        TimeControlText,
    ));
}
