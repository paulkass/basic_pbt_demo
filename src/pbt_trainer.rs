use basic_pbt_demo::{VectorToScalar, VectorToVector};
use std::thread::JoinHandle;
use std::vec::Vec;
use rulinalg::vector::Vector;

use crate::barrier_wrapper::BarrierWrapper;

pub struct PBTTrainer<'a, T: Copy> {
    pub heuristic: &'a VectorToScalar<T>,
    pub derivative: &'a VectorToVector<T>,
    pub workers: i32,
    pub iterations: i32,
    handles: Vec<JoinHandle<Points>>
}

impl<'a, T: Copy> PBTTrainer<'a, T> {
    pub fn new(heuristic: &'a VectorToScalar<T>,
            derivative: &'a VectorToVector<T>,
            workers: i32,
            iterations: i32) -> PBTTrainer<'a, T> {
        PBTTrainer {
            heuristic,
            derivative,
            workers,
            iterations,
            handles: Vec::with_capacity(workers as usize)
        }
    }

    // Assume constant learning rate
    pub fn start(&self, start_vector: Vector<f64>, learning_rate: f64) -> Vec<Vec<Points>> {
        let barriers = BarrierWrapper::new(self.iterations, self.workers + 1);

        for _ in 0..self.workers {
            let barrier = barriers.clone();
        }

        Vec::new()
    }
}

pub struct Points {
    points: Vec<TrainingEvent>
}

pub enum TrainingEvent {
    Point(Vector<f64>), // theta
    Exploit(Vector<f64>, Vector<f64>), // theta and new h
    Explore(Vector<f64>, Vector<f64>), // theta and new h
}
