use rulinalg::vector::Vector;
use basic_pbt_demo::{CommonFunctions};
use crate::pbt_trainer::PBTTrainer;

mod pbt_trainer;
mod barrier_wrapper;

fn main() {
  let start_vector = Vector::new(vec![0.9, 0.9]);

  let eval_function = &CommonFunctions::example_fn;
  let derivative_function = &CommonFunctions::example_derivative;
  let actual_function = &CommonFunctions::actual_fn;

  let pbt = PBTTrainer::new(eval_function, derivative_function, 2, 4);

  let results = pbt.start(start_vector, 0.1);
}
