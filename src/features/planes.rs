use crate::features::shape::Shape;

use super::{
    intersections::Intersection, materials::Material, matrice::Matrice, rays::Ray, tuple::Tuple,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Plane {
    pub material: Material,
    pub transform: Matrice,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            material: Material::new(),
            transform: Matrice::identity_matrix(4),
        }
    }

    pub fn normal_at(&self, _point: Tuple) -> Tuple {
        self.transform.clone() * Tuple::vector(0.0, 1.0, 0.0)
    }

    pub fn set_transform(&mut self, transform: Matrice) {
        self.transform = transform
    }

    pub fn transform(&self) -> Matrice {
        self.transform.clone()
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < 0.00001 {
            return vec![];
        }
        let t = (-ray.origin.y) / ray.direction.y;
        vec![Intersection::new(t, Shape::Plane(self.clone()))]
    }
}