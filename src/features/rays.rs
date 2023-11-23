use super::{matrice::Matrice, tuple::Tuple};

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}
impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }
    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }
}

pub fn transform(r: &Ray, m: Matrice) -> Ray {
    Ray::new(m.clone() * r.origin, m.clone() * r.direction)
}