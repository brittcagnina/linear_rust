pub struct Vector {
    component: Vec<f64>,
}

impl Vector {
    pub fn new(v: Vec<f64>) -> Vector { Vector { component: v, } }
    pub fn get(&self, n: i64)  -> f64 { 0f64 }
    pub fn append(&self, x: f64) -> Vector { Vector{ component: vec![0f64] } }
    pub fn remove(&self, n: i64) -> Vector { Vector{ component: vec![0f64] } }
    pub fn dot(&self, v: Vec<f64>) -> f64 { 0f64 }
    pub fn add(&self, v: Vec<f64>) -> Vector { Vector{ component: vec![0f64] } }
    pub fn view(&self) { }
    pub fn len(&self) -> usize { self.component.len() } 
}
