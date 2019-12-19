use basic_pbt_demo::{VectorToScalar, VectorToVector};

pub struct PBTTrainer<'a, T: Copy> {
    pub heuristic: &'a VectorToScalar<T>,
    pub derivative: &'a VectorToVector<T>,
    pub workers: i32
}

impl<'a, T: Copy> PBTTrainer<'a, T> {
    pub fn new(heuristic: &'a VectorToScalar<T>,
            derivative: &'a VectorToVector<T>,
            workers: i32) -> PBTTrainer<'a, T> {
        PBTTrainer {
            heuristic,
            derivative,
            workers
        }
    }
}
