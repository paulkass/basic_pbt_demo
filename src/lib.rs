use rulinalg::vector::Vector;

pub struct Evaluation<'a, T: Copy> {
   function: &'a Fn(Vector<T>, Vector<T>) -> T
}

impl<'a, T: Copy> Evaluation<'a, T> {
   pub fn new<F>(f: &'a F) -> Evaluation<T>
      where F: Fn(Vector<T>, Vector<T>) -> T {
      Evaluation {
         function: f
      }
   }

   pub fn call(&self, a: Vector<T>, b: Vector<T>) -> T {
      (self.function)(a, b)
   }
}

pub struct CommonFunctions {}

impl CommonFunctions {
   pub fn example_fn(theta: Vector<f64>, h: Vector<f64>) -> f64 {
      let theta_squared = theta.clone();
      1.2 - (h.dot(&theta_squared.apply(&|x| { x.powi(2) })))
   }
}

#[test]
fn test1() {
   let eval = Evaluation::new(&CommonFunctions::example_fn);
   assert_eq!(eval.call(Vector::new(vec![0.5, 0.5]), Vector::new(vec![1.0, 1.0])),
      0.7);
}