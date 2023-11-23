use super::tuple::Tuple;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Tuple,
}

impl Light {
    pub fn new(position: Tuple, intensity: Tuple) -> Self {
        Light {
            position,
            intensity,
        }
    }
}