use std::ops::{Add, Mul};
use traits;

pub struct Vector<T: traits::LinrustTrait<T>> {
    component: Vec<T>,
}

impl<T: traits::LinrustTrait<T>> Vector<T> {
    pub fn new(v: Vec<T>) -> Vector<T> { Vector { component: v, } }
    pub fn clone(&self) -> Self {
	Vector::new(self.component.clone()) 
    }
    pub fn get(&self, n: usize) -> T { self.component[n].clone() }
    pub fn append(&mut self, x: T) { self.component.push(x); }
    pub fn dot(self, v: Vector<T>) -> T { self * v }
    pub fn add(self, v: Vector<T>) -> Vector<T> { self + v }
    pub fn view(&self) {
        print!["< "];
        for x in 0..self.len() {
            if x == self.len() - 1 {
                print!{ "{}", self.get(x) };
            } else {
                print!{ "{}, ", self.get(x) };
            }
        }
        print![" >\n"];
    }
    pub fn dim(&self) -> usize { self.component.len() } 
    pub fn len(&self) -> usize { 
        self.component.len() 
    } 
}


// Addition Operator Overloads
impl<T: traits::LinrustTrait<T>> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;
    fn add(self, other: Vector<T>) -> Vector<T> {
        let mut vec: Vec<T> = Vec::new();
        for x in 0..self.len() {
            vec.push(self.get(x) + other.get(x));
        }
        Vector::new(vec)
    }
}

// Multiplication Operator Overloads
impl<T: traits::LinrustTrait<T>> Mul<Vector<T>> for Vector<T> {
    type Output = T;
    fn mul(self, other: Vector<T>) -> T {
        //TODO: Fix this init logic
        let mut ret: T = self.get(0) * other.get(0);
        for x in 1..self.len() {
            ret = ret + self.get(x) * other.get(x)
        }
        ret
    }
}
