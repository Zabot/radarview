use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res, ResMut, Resource};
use bevy::text::Text;

#[derive(Component)]
pub struct TimeSeries<T>
where
    T: Clone + Component,
{
    history: Vec<(f64, T)>,
}

impl<T> TimeSeries<T>
where
    T: Clone + Component,
{
    pub fn new(history: Vec<(f64, T)>) -> Self {
        Self { history }
    }

    pub fn before(&self, time: f64) -> impl Iterator<Item = &T> {
        self.history
            .iter()
            .filter_map(move |(t, v)| if *t <= time { Some(v) } else { None })
    }

    /// Get the value closest to the given time without going past it
    pub fn at_or_before(&self, time: f64) -> Option<&T> {
        self.history
            .iter()
            .filter_map(move |(t, v)| {
                if *t <= time && (*t - time) > -0.01 {
                    Some(v)
                } else {
                    None
                }
            })
            .last()
    }
}

#[derive(Resource)]
pub struct Time(pub f64);

#[derive(Resource, Debug)]
pub struct TimeFlow {
    pub delta: f64,
    pub paused: bool,
}

impl Default for TimeFlow {
    fn default() -> Self {
        Self {
            delta: 0.01,
            paused: false,
        }
    }
}

#[derive(Component, Debug)]
pub struct Active(pub bool);

// Updates the state of all elements to be the latest value prior to the given time
pub fn update_current_time<T>(
    time: Res<Time>,
    mut query: Query<(&TimeSeries<T>, &mut T, &mut Active)>,
) where
    T: Clone + Component,
{
    for (time_series, mut current, mut active) in query.iter_mut() {
        match time_series.at_or_before(time.0) {
            None => *active = Active(false),
            Some(value) => {
                *active = Active(true);
                *current = value.clone();
            }
        }
    }
}

// Updates the state of all elements to be the latest value prior to the given time
pub fn advance_time(mut time: ResMut<Time>, flow: Res<TimeFlow>) {
    if flow.paused {
        return;
    }

    time.0 += flow.delta
}

#[derive(Component)]
pub struct ElapsedText;
pub fn elapsed_text_update(time: Res<Time>, mut query: Query<&mut Text, With<ElapsedText>>) {
    for mut text in &mut query {
        text.sections[0].value = format!("Elapsed: {:.3}s", time.0);
    }
}
