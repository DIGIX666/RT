use super::shape::Shape;
#[derive(Clone, PartialEq, Debug)]
pub struct Intersection {
    pub t: f32,
    pub s: Shape,
}

impl Intersection {
    pub fn new(t: f32, s: Shape) -> Self {
        Self { t, s }
    }
}

pub fn intersections(xs: &mut [Intersection]) -> Vec<Intersection> {
    xs.sort_by(|a, b| a.t.total_cmp(&b.t));
    xs.to_vec()
}

pub fn hit(xs: Vec<Intersection>) -> Option<Intersection> {
    for i in xs {
        if i.t > 0.0 {
            return Some(i.clone());
        }
    }
    None
}


pub mod computations {

    use crate::features::{rays::Ray, shape::Shape, tuple::Tuple};

    use super::Intersection;

    pub struct Computation {
        pub t: f32,
        pub object: Shape,
        pub point: Tuple,
        pub eyev: Tuple,
        pub normalv: Tuple,
        pub reflectv: Tuple,
        pub inside: bool,
        pub over_point: Tuple,
        pub under_point: Tuple,
        pub n1: f32,
        pub n2: f32,
    }

    impl Computation {
        pub fn new(i: &Intersection, r: &Ray, xs: &[Intersection]) -> Self {
            let mut n1 = 1.0;
            let mut n2 = 1.0;
            let mut containers: Vec<Shape> = vec![];
            for x in xs.iter() {
                if *i == *x {
                    if let Some(l) = containers.last() {
                        n1 = l.material().refractive_index;
                    }
                }
                if let Some(index) = containers.iter().position(|a| *a == x.s) {
                    containers.remove(index);
                } else {
                    containers.push(x.s.clone())
                }
                if *i == *x {
                    if let Some(l) = containers.last() {
                        n2 = l.material().refractive_index;
                    }
                    break;
                }
            }
            let mut normalv = i.s.normal_at(r.position(i.t));
            let mut inside = false;
            if normalv.dot(&-(r.direction)) < 0.0 {
                inside = true;
                normalv = -normalv;
            }
            Computation {
                t: i.t,
                object: i.s.clone(),
                point: r.position(i.t),
                eyev: -(r.direction),
                normalv,
                reflectv: r.direction.reflect(&normalv),
                inside,
                over_point: r.position(i.t) + (normalv * 0.0001),
                under_point: r.position(i.t) - (normalv * 0.0001),
                n1,
                n2,
            }
        }
    }
}