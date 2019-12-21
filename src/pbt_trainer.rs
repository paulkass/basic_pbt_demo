use std::ops::{Add};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::vec::Vec;

use basic_pbt_demo::{Vector, VectorToScalar, VectorToVector};
use spmc::Receiver;

pub struct PBTTrainer {
    pub heuristic: Arc<VectorToScalar<f64>>,
    pub derivative: Arc<VectorToVector<f64>>,
    pub explore: Arc<Fn(State) -> State + Sync + Send>,
    pub workers: i32,
    pub iterations: i32,
}

impl PBTTrainer {
    pub fn new(heuristic: Arc<VectorToScalar<f64>>,
               derivative: Arc<VectorToVector<f64>>,
               explore: Arc<Fn(State) -> State + Sync + Send>,
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
        let (thread_sender, main_receiver) = channel();
        let (mut main_sender, thread_receiver) = spmc::channel();

        let mut handles = Vec::new();
        let iterations = self.iterations;
        for t in 0..self.workers {
            let rx: Receiver<State>= thread_receiver.clone();
            let sender = thread_sender.clone();
            let explore = self.explore.clone();
            let derivative = self.derivative.clone();
            let theta = start_vector.clone();

            let handle = thread::spawn(move || -> Points {
                let mut points: Vec<TrainingEvent> = vec![];

                let mut cur_state = State::default();


                for i in 0..iterations {
                    let state;
                    if i == 0 {
                        let mut initial_h = Vector::zeros();
                        *(initial_h.get_mut((t%2) as usize)) = 1.0;
                        let initial_state = State {
                            theta,
                            h: initial_h.clone(),
                        };
                        state = initial_state;
                    } else {
                        state = rx.recv().expect("Could not receive a state from the main thread. Aborting.");
                    }

                    // Exploit or Explore
                    if state != cur_state {
                        cur_state = state;
                        points.push(TrainingEvent::Exploit(cur_state.theta.clone(), cur_state.h.clone()))
                    } else {
                        cur_state = (explore)(state);
                        points.push(TrainingEvent::Explore(cur_state.theta.clone(), cur_state.h.clone()))
                    }

                    // Apply Gradient Descent
                    for _ in 0..4 {
                        let d = (derivative)(cur_state.theta, cur_state.h) * learning_rate;
                        //println!("Derivative is {:?}", d);
                        cur_state.theta = cur_state.theta.add(d);
                        points.push(TrainingEvent::Point(cur_state.theta.clone()))
                    }

                    // Communicate results to main thread
                    sender.send(cur_state.clone()).expect("Could not send communications to main thread");
                }

                Points {
                    points
                }
            });

            handles.push(handle);
        }
        let handles = handles;

        for _ in 0..(self.iterations - 1) {
            let mut results = Vec::new();
            for _ in 0..self.workers {
                results.push(main_receiver.recv().unwrap())
            }

            results.sort_by(|a, b| {
                let val1: f64 = (self.heuristic)(a.theta, a.h);
                let val2: f64 = (self.heuristic)(b.theta, b.h);
                val1.partial_cmp(&val2).unwrap()
            });

            let best = results.first_mut().unwrap();
            for _ in 0..self.workers {
                main_sender.send(best.clone()).expect("Could not send communications to sub-threads");
            }
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join().unwrap());
        }
        results
    }
}

#[derive(Debug)]
pub struct Points {
    pub points: Vec<TrainingEvent>
}

#[derive(Debug)]
pub enum TrainingEvent {
    Point(Vector<f64>),
    // theta
    Exploit(Vector<f64>, Vector<f64>),
    // theta and new h
    Explore(Vector<f64>, Vector<f64>), // theta and new h
}

#[derive(Debug)]
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
