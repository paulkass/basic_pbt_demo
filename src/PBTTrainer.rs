use basic_pbt_demo::{VectorToScalar, VectorToVector};

pub struct PBTTrainer<'a, T: Copy> {
    pub heuristic: &'a VectorToScalar<T>,
    pub derivative: &'a VectorToVector<T>,
    pub workers: i32
}
