use uuid::Uuid;
// use crate::features::shape::Shape;

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
            minimum: -f32::INFINITY,
            maximum: f32::INFINITY,
            closed: false,
        }
    }

    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        // Exemple simplifié. Vous devez implémenter le calcul correct des intersections.
        vec![]
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        // Exemple simplifié. Vous devez implémenter le calcul correct des normales.
        Tuple::vector(0.0, 1.0, 0.0)
    }

    // Ajoutez d'autres méthodes au besoin, comme set_transform, etc.
}

fn check_cap(ray: &Ray, t: f32) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    (x * x + z * z) <= 1.0
}

