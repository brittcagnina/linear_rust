use std::ops::Add;
use std::ops::Mul;

pub struct Vector {
    component: Vec<f64>,
}

impl Vector {
    pub fn new(v: Vec<f64>) -> Vector { Vector { component: v, } }
    pub fn clone(&self) -> Self {
	Vector::new(self.component.clone()) 
    }
    pub fn get(&self, n: usize)  -> f64 { self.component[n] }
    pub fn append(&mut self, x: f64) { self.component.push(x); }
    pub fn remove(&self, n: i64) -> Vector { Vector{ component: vec![0f64] } }
    pub fn dot(self, v: Vector) -> f64 { 
        self * v 
    }
    pub fn add(&self, v: Vector) -> Vector { 
        Vector{ component: vec![0f64] } 
    }
    pub fn view(&self) {
        print!["<"];
        for x in 0..self.len() {
            print!["{},", x];
        }
        print![">\n"];
    }
    pub fn dim(&self) -> usize { self.component.len() } 
    pub fn len(&self) -> usize { 
        self.component.len() 
    } 
}

// Addition Operator Overloads
add_overload!(Vector, Vector, Vector, self, object, { 
    if self.len() != object.len() {
        Vector::new(vec![0.0]) //TODO fix ret value
    } else {
        let mut vec: Vec<f64> = Vec::new();
        for x in 0..self.len() {
            vec.push(self.get(x) + object.get(x));
        }
        Vector::new(vec) 
    }
});

// Multiplication Operator Overloads
mul_overload!(Vector, Vector, f64, self, object, {
    if self.len() != object.len() {
        0.0 //TODO fix ret value
    } else {
        let mut ret: f64 = 0.0;
        for x in 0..self.len() {
            ret += self.get(x) * object.get(x)
        }
        ret
    }
});
