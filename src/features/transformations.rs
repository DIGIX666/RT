use super::{matrice::Matrice, tuple::Tuple};

pub fn translation(x: f32, y: f32, z: f32) -> Matrice {
    let mut out = Matrice::identity_matrix(4);
    out.write_element(0, 3, x);
    out.write_element(1, 3, y);
    out.write_element(2, 3, z);
    out
}
pub fn scaling(x: f32, y: f32, z: f32) -> Matrice {
    let mut out = Matrice::identity_matrix(4);
    out.write_element(0, 0, x);
    out.write_element(1, 1, y);
    out.write_element(2, 2, z);
    out
}

pub fn rotation_x(rad: f32) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(1, 1, rad.cos());
    matrice.write_element(1, 2, -rad.sin());
    matrice.write_element(2, 1, rad.sin());
    matrice.write_element(2, 2, rad.cos());
    matrice
}

pub fn rotation_y(rad: f32) -> Matrice {
    let mut matrice = Matrice::identity_matrix(4);
    matrice.write_element(0, 0, rad.cos());
    matrice.write_element(0, 2, rad.sin());
    matrice.write_element(2, 0, -rad.sin());
    matrice.write_element(2, 2, rad.cos());
    matrice
}

pub fn view_transformation(from: Tuple, to: Tuple, up: Tuple) -> Matrice {
    let forward = (to - from).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);
    Matrice {
        size: 4,
        matrice: vec![
            vec![left.x, left.y, left.z, 0.0],
            vec![true_up.x, true_up.y, true_up.z, 0.0],
            vec![-forward.x, -forward.y, -forward.z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ],
    } * translation(-from.x, -from.y, -from.z)
}