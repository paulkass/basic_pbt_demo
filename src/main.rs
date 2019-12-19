
mod PBTTrainer;

use rulinalg::vector::Vector;
use basic_pbt_demo::{Evaluation, CommonFunctions};

fn main() {
  let start_vector = Vector::new(vec![0.9, 0.9]);

  let eval_function = Evaluation::new(&CommonFunctions::example_fn);

  let actual_function = Evaluation::new(&CommonFunctions::actual_fn);

}
