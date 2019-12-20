use std::ops::{Add, Mul};

pub type VectorToScalar<T> = dyn Fn(Vector<T>, Vector<T>) -> T + Sync + Send;
pub type VectorToVector<T> = dyn Fn(Vector<T>, Vector<T>) -> Vector<T> + Sync + Send;

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
      let h = Vector { a: 1.0, b: 1.0 };
      1.2 - (h.dot(&theta_squared.apply(&|x| { x.powi(2) })))
   }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vector<T: Add + Mul + Copy + Default> {
   pub a: T,
   pub b: T,
}

impl<T: Add<Output = T> + Mul + Copy + Default> Add for Vector<T> {
   type Output = Self;

   fn add(self, rhs: Self) -> Self {
      let a = self.a + rhs.a;
      let b = self.b + rhs.b;
      Self { a, b }
   }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Default> Mul<T> for Vector<T> {
   type Output = Self;

   fn mul(self, rhs: T) -> Self::Output {
      let a = self.a * rhs;
      let b = self.b * rhs;
      Self::Output { a, b }
   }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + Default> Vector<T> {
   pub fn dot(&self, rhs: &Self) -> T {
      self.a * rhs.a + self.b * rhs.b
   }

   pub fn apply(&self, f: &Fn(T) -> T) -> Vector<T> {
      Vector {
         a: (f)(self.a),
         b: (f)(self.b),
      }
   }

   pub fn get_mut(&mut self, i: usize) -> &mut T {
       match i {
          0 => &mut self.a,
          1 => &mut self.b,
          _ => panic!("Index out of bounds")
       }
   }

   pub fn zeros() -> Vector<T> {
      Vector {
         a: T::default(),
         b: T::default(),
      }
   }
}

#[test]
fn test1() {
   let eval = &CommonFunctions::example_fn;
   assert_eq!((eval)(Vector { a: 0.5, b: 0.5 }, Vector { a: 1.0, b: 1.0 }),
      0.7);
}