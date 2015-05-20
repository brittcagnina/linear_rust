use std::ops::{Add, Mul};
use std::fmt::Display;

pub trait LinrustTrait<T>: Clone + Display + Add<T, Output = T> + Mul<T, Output = T> { } 
impl<T> LinrustTrait<T> for T where T: Clone + Display + Add<T, Output = T> + Mul<T, Output = T> { }
