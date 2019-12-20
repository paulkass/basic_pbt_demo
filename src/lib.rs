use rulinalg::vector::Vector;

pub type VectorToScalar<T> = dyn Fn(Vector<T>, Vector<T>) -> T;
pub type VectorToVector<T> = dyn Fn(Vector<T>, Vector<T>) -> Vector<T>;

pub struct CommonFunctions {}

impl CommonFunctions {
   pub fn example_fn(theta: Vector<f64>, h: Vector<f64>) -> f64 {
      let theta_squared = theta.clone();
      1.2 - (h.dot(&theta_squared.apply(&|x| { x.powi(2) })))
   }

   pub fn example_derivative(_: Vector<f64>, h: Vector<f64>) -> Vector<f64> {
      h
   }

   pub fn actual_fn(theta: Vector<f64>, _: Vector<f64>) -> f64 {
      let theta_squared = theta.clone();
      let h = Vector::new(vec![1.0, 1.0]);
      1.2 - (h.dot(&theta_squared.apply(&|x| { x.powi(2) })))
   }
}

#[test]
fn test1() {
   let eval = &CommonFunctions::example_fn;
   assert_eq!((eval)(Vector::new(vec![0.5, 0.5]), Vector::new(vec![1.0, 1.0])),
      0.7);
}