use std::sync::{Barrier, Arc};

pub struct BarrierWrapper {
    barriers: Vec<Arc<Barrier>>
}

impl BarrierWrapper {
    pub fn new(number: i32, thread_count: i32) -> BarrierWrapper {
        let mut barriers: Vec<Arc<Barrier>> = Vec::new();
        for i in 0..number {
            barriers.push(Arc::new(Barrier::new(thread_count as usize)));
        }
        BarrierWrapper {
            barriers
        }
    }

    pub fn get(&self, i: usize) -> &Arc<Barrier> {
        self.barriers.get(i).expect(format!("Index out of bounds: {}", i).as_str())
    }
}

impl Clone for BarrierWrapper {
    fn clone(&self) -> Self {
        let mut barrier_copy = Vec::new();
        for b in self.barriers.iter() {
           barrier_copy.push(b.clone());
        }
        BarrierWrapper {
            barriers: barrier_copy
        }
    }
}