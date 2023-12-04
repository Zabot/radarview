use anyhow::Result;
use serde::Deserialize;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, fs::File};

use crate::beam::{BeamBundle, BeamState};
use crate::polar::PolarVec3;
use crate::state::State;
use crate::timeseries::{Active, TimeSeries};

const MAX_RANGE: f32 = 200_000.0;

#[derive(Debug, Deserialize)]
pub struct Beam {
    pub width: f32,
    pub position: [f32; 2],
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub state: [f32; 6],
    pub uncertainty: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    pub elapsed: f64,
    pub truths: HashMap<String, [f32; 6]>,
    pub tracks: HashMap<String, Track>,
    pub beams: Vec<Beam>,
}

pub struct SimulationRun {
    steps: Vec<Step>,
}

impl SimulationRun {
    pub fn new(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut steps = Vec::new();
        loop {
            let mut buf = String::new();
            let size = reader.read_line(&mut buf)?;
            if size == 0 {
                break;
            }
            let step = serde_json::from_str(&buf)?;
            steps.push(step);
        }

        Ok(Self { steps })
    }

    pub fn truths(&self) -> Vec<(TimeSeries<State>, State, Active)> {
        let mut truth_ids = HashSet::new();
        for step in self.steps.iter() {
            truth_ids.extend(step.truths.keys())
        }

        let mut truths = Vec::with_capacity(truth_ids.len());
        for truth_id in truth_ids.iter() {
            let mut history = Vec::new();
            for step in self.steps.iter() {
                if let Some(truth) = step.truths.get(truth_id.as_str()) {
                    history.push((
                        step.elapsed,
                        State::default()
                            //.with_xyz(truth[0], truth[2], truth[4])
                            .with_xyz(truth[2], truth[4], truth[0])
                            .with_vel(truth[1], truth[3], truth[5]),
                    ))
                }
            }
            let first = history[0].1.clone();
            truths.push((TimeSeries::new(history), first, Active(false)))
        }

        truths
    }

    pub fn beams(&self) -> Vec<BeamBundle> {
        let mut beams = Vec::new();
        for index in 0..4 {
            let mut history = Vec::new();
            for step in self.steps.iter() {
                let beam = &step.beams[index];
                let beam_pos = PolarVec3::new(MAX_RANGE, beam.position[0], beam.position[1]);
                let state = BeamState {
                    width: beam.width,
                    target: beam_pos,
                    index,
                };
                history.push((step.elapsed, state))
            }
            beams.push(BeamBundle {
                state: history[0].1.clone(),
                active: Active(true),
                history: TimeSeries::new(history),
            })
        }
        beams
    }
}
