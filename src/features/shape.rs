use super::{
    cube::Cube,
    intersections::Intersection,
    materials::Material,
    matrice::Matrice,
    planes::Plane,
    cylinders::Cylinder,
    rays::{transform, Ray},
    spheres::Sphere,
    tuple::Tuple,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
        && self.transform == other.transform
        && self.material == other.material
        && (self.minimum - other.minimum).abs() < f32::EPSILON
        && (self.maximum - other.maximum).abs() < f32::EPSILON
        && self.closed == other.closed
    }
}


impl Shape {
    pub fn intersect(&self, r: &Ray) -> Vec<Intersection> {
        let ray = transform(r, self.transform().inverse().unwrap());
        match self {
            Shape::Sphere(s) => s.intersect(&ray),
            Shape::Plane(p) => p.intersect(&ray),
            Shape::Cube(c) => c.intersect(&ray),
            Shape::Cylinder(cy) => cy.intersect(&ray),
        }
    }


    pub fn set_transform(&mut self, t: Matrice) {
        match self {
            Shape::Sphere(s) => s.set_transform(t),
            Shape::Plane(p) => p.set_transform(t),
            Shape::Cube(c) => c.transform = t,
            Shape::Cylinder(cy) => cy.transform = t,
        }
    }


    pub fn transform(&self) -> Matrice {
        match self {
            Shape::Sphere(s) => s.transform.clone(),
            Shape::Plane(p) => p.transform(),
            Shape::Cube(c) => c.transform.clone(),
            Shape::Cylinder(cy) => cy.transform.clone(),
        }
    }


    pub fn normal_at(&self, point: Tuple) -> Tuple {
        match self {
            Shape::Sphere(s) => s.normal_at(point),
            Shape::Plane(p) => p.normal_at(point),
            Shape::Cube(c) => c.normal_at(point),
            Shape::Cylinder(cy) => cy.normal_at(point),
        }
    }


    pub fn material(&self) -> Material {
        match self {
            Shape::Sphere(s) => s.material.clone(),
            Shape::Plane(p) => p.material.clone(),
            Shape::Cube(c) => c.material.clone(),
            Shape::Cylinder(cy) => cy.material.clone(),
        }
    }


    pub fn set_material(&mut self, m: Material) {
        match self {
            Shape::Sphere(s) => s.material = m,
            Shape::Plane(_) => {}
            Shape::Cube(_) => {}
            Shape::Cylinder(_) => {}
        }
    }
}