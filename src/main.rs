use basic_pbt_demo::{CommonFunctions, Vector};
use crate::pbt_trainer::{PBTTrainer, State};
use std::sync::Arc;

mod pbt_trainer;
mod barrier_wrapper;

fn main() {
  let start_vector = Vector { a: 0.5, b: 0.5 };

  let eval_function = Arc::new(CommonFunctions::example_fn);
  let derivative_function = Arc::new(CommonFunctions::example_derivative);
  let actual_function = Arc::new(CommonFunctions::actual_fn);

  let mut pbt = PBTTrainer::new(eval_function, derivative_function, Arc::new(|state: &State| {
    state.clone()
  }), 2, 4);

  let results = pbt.start(start_vector, 0.1);
}
