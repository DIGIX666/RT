use uuid::Uuid;
use crate::features::shape::Shape;

use super::{
    intersections::{Intersection},
    materials::Material,
    matrice::Matrice,
    rays::Ray,
    tuple::Tuple,
};

#[derive(Debug, Clone)]
pub struct Cylinder {
    pub id: Uuid,
    pub transform: Matrice,
    pub material: Material,
    pub minimum: f32,
    pub maximum: f32,
    pub closed: bool,
}

impl Cylinder {
    pub fn new() -> Self {
        Self {
            material: Material::new(),
            id: Uuid::new_v4(),
            transform: Matrice::identity_matrix(4),
            minimum: f32::NEG_INFINITY,
            maximum: f32::INFINITY,
            closed: false,
        }
    }
    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let a = r.direction.x.powi(2) + r.direction.z.powi(2);
        if a.abs() < f32::EPSILON {
            return self.intersect_caps(r);
        }
        let b = 2.0 * r.origin.x * r.direction.x + 2.0 * r.origin.z * r.direction.z;
        let c = r.origin.x.powi(2) + r.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return vec![];
        }
        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        let mut xs = vec![];
        let y0 = r.origin.y + t0 * r.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(Intersection::new(t0, Shape::Cylinder(self.clone())));
        }
        let y1 = r.origin.y + t1 * r.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(Intersection::new(t1, Shape::Cylinder(self.clone())));
        }
        xs.append(&mut self.intersect_caps(r));
        xs
    }
    pub fn intersect_caps(&self, r: &Ray) -> Vec<Intersection> {
        let mut xs = vec![];
        if !self.closed || r.direction.y.abs() < f32::EPSILON {
            return xs;
        }
        let t = (self.minimum - r.origin.y) / r.direction.y;
        if check_cap(r, t) {
            xs.push(Intersection::new(t, Shape::Cylinder(self.clone())));
        }
        let t = (self.maximum - r.origin.y) / r.direction.y;
        if check_cap(r, t ) {
            xs.push(Intersection::new(t, Shape::Cylinder(self.clone())));
        }
        xs
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < 1.0 && point.y >= self.maximum - f32::EPSILON {
            Tuple::vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.minimum + f32::EPSILON {
            Tuple::vector(0.0, -1.0, 0.0)
        } else {
            Tuple::vector(point.x, 0.0, point.z)
        }
    }
}

fn check_cap(r: &Ray, t: f32) -> bool {
    let x = r.origin.x + t * r.direction.x;
    let z = r.origin.z + t * r.direction.z;
    x.powi(2) + z.powi(2) <= 1.0
}