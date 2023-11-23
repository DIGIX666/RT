use std::fmt::Debug;

use super::{matrice::Matrice, shape::Shape, tuple::Tuple};

#[derive(PartialEq, Debug, Clone)]
pub enum Pattern {
    Checker(Checker),
}

impl Pattern {
    pub fn at_object(&self, shape: &Shape, point: &Tuple) -> Tuple {
        let object_point = shape.transform().inverse().unwrap() * *point;
        let pattern_point = self.transform().inverse().unwrap() * object_point;
        match self {
            Pattern::Checker(checker) => checker.at(&pattern_point),
        }
    }

    pub fn transform(&self) -> Matrice {
        match self {
            Pattern::Checker(checker) => checker.transform.clone(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Checker {
    a: Tuple,
    b: Tuple,
    transform: Matrice,
}

impl Checker {
    pub fn new(_a: Tuple, _b: Tuple) -> Self {
        Self {
            a: Tuple::color(1.0, 1.0, 1.0), // Blanc
            b: Tuple::color(1.0, 0.0, 0.0), // Rouge
            transform: Matrice::identity_matrix(4),
        }
    }
   pub fn at(&self, point: &Tuple) -> Tuple {
        // Calcul pour un motif à damier plus fin
        let check_sum = (point.x.floor() as i32 % 2 == 0) ^ 
                        (point.y.floor() as i32 % 2 == 0) ^ 
                        (point.z.floor() as i32 % 2 == 0);

        if check_sum {
            return self.a;
        }
        self.b
    }
}