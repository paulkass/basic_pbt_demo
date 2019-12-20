use std::ops::{Add, Mul, MulAssign};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;
use std::vec::Vec;

use spmc_buffer::{SPMCBuffer, SPMCBufferOutput};

use basic_pbt_demo::{Vector, VectorToScalar, VectorToVector};

pub struct PBTTrainer {
    pub heuristic: Arc<VectorToScalar<f64>>,
    pub derivative: Arc<VectorToVector<f64>>,
    pub explore: Arc<Fn(&State) -> State + Sync + Send>,
    pub workers: i32,
    pub iterations: i32,
}

impl PBTTrainer {
    pub fn new(heuristic: Arc<VectorToScalar<f64>>,
               derivative: Arc<VectorToVector<f64>>,
               explore: Arc<Fn(&State) -> State + Sync + Send>,
               workers: i32,
               iterations: i32) -> PBTTrainer {
        PBTTrainer {
            heuristic,
            derivative,
            explore,
            workers,
            iterations,
        }
    }

    // Assume constant learning rate
    pub fn start(&mut self, start_vector: Vector<f64>, learning_rate: f64) -> Vec<Points> {
        //let (thread_sender, main_receiver) = channel();
        let (mut main_sender, mut thread_receiver) = SPMCBuffer::with_default(self.workers as usize).split();

        let mut handles = Vec::new();
        let iterations = self.iterations;
        for i in 0..self.workers {
            let mut rx: SPMCBufferOutput<State> = thread_receiver.clone();
            let explore = self.explore.clone();
            let derivative = self.derivative.clone();

            let handle = thread::spawn(move || -> Points {
                let mut points: Vec<TrainingEvent> = vec![];

                let mut cur_state = State::default();

                for _ in 0..iterations {
                    let state = rx.read();

                    if *state != cur_state {
                        cur_state = state.clone();
                        points.push(TrainingEvent::Exploit(cur_state.theta.clone(), cur_state.h.clone()))
                    } else {
                        cur_state = (explore)(state);
                        points.push(TrainingEvent::Explore(cur_state.theta.clone(), cur_state.h.clone()))
                    }

                    for _ in 0..4 {
                        cur_state.theta.add((derivative)(cur_state.theta, cur_state.h) * learning_rate);
                        points.push(TrainingEvent::Point(cur_state.theta.clone()))
                    }
                }

                Points {
                    points
                }
            });

            handles.push(handle);
        }
        let handles = handles;

        //let theta = start_vector.clone();
        // One hot encode h
        //let mut h: Vector<f64> = Vector::zeros(2);
        //*h.index_mut((i%2) as usize) = 1.0;

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().unwrap());
        }
        results
    }
}

pub struct Points {
    pub points: Vec<TrainingEvent>
}

pub enum TrainingEvent {
    Point(Vector<f64>),
    // theta
    Exploit(Vector<f64>, Vector<f64>),
    // theta and new h
    Explore(Vector<f64>, Vector<f64>), // theta and new h
}

pub struct State {
    pub theta: Vector<f64>,
    pub h: Vector<f64>,
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            theta: self.theta.clone(),
            h: self.h.clone(),
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.theta == other.theta && self.h == other.h
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            theta: Vector::zeros(),
            h: Vector::zeros(),
        }
    }
}
