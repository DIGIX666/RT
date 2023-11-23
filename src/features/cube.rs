use std::f32::INFINITY;

use uuid::Uuid;

use crate::features::shape::Shape;

use super::{
    intersections::Intersection,
    materials::Material,
    matrice::Matrice,
    rays::Ray,
    tuple::Tuple,
};

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Cube {
    pub id: Uuid,
    pub transform: Matrice,
    pub material: Material,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            material: Material::new(),
            id: Uuid::new_v4(),
            transform: Matrice::identity_matrix(4),
        }
    }

    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = check_axis(r.origin.x, r.direction.x);
        let (ytmin, ytmax) = check_axis(r.origin.y, r.direction.y);
        let (ztmin, ztmax) = check_axis(r.origin.z, r.direction.z);
        let tmin = [xtmin, ytmin, ztmin]
            .into_iter()
            .max_by(|x, y| x.total_cmp(y))
            .unwrap();
        let tmax = [xtmax, ytmax, ztmax]
            .into_iter()
            .min_by(|x, y| x.total_cmp(y))
            .unwrap();
        if tmin > tmax {
            return vec![];
        }
        vec![
            Intersection::new(tmin, Shape::Cube(self.clone())),
            Intersection::new(tmax, Shape::Cube(self.clone())),
        ]
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let maxc = [point.x.abs(), point.y.abs(), point.z.abs()]
            .into_iter()
            .max_by(|x, y| x.total_cmp(y))
            .unwrap();
        if maxc == point.x.abs() {
            self.transform.clone() * Tuple::vector(point.x, 0.0, 0.0)
        } else if maxc == point.y.abs() {
            self.transform.clone() * Tuple::vector(0.0, point.y, 0.0)
        } else {
            self.transform.clone() * Tuple::vector(0.0, 0.0, point.z)
        }
    }
}

fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;
    let mut tmin = tmin_numerator * INFINITY;
    let mut tmax = tmax_numerator * INFINITY;
    if direction.abs() >= 0.0001 {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    }
    if tmin > tmax {
        return (tmax, tmin);
    }
    (tmin, tmax)
}