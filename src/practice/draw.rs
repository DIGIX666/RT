use std::f32::consts::PI;

use crate::features::{
    camera::cameras::Camera,
    cube::Cube,
    lights::Light,
    materials::Material,
    patterns::{Checker,Pattern},
    planes::Plane,
    cylinders::Cylinder,
    shape::Shape,
    spheres::Sphere,
    transformations::{
        rotation_x, rotation_y,scaling,translation, view_transformation,
    },
    tuple::Tuple,
    world::World,
};

pub fn drawing() {
    let light_position = Tuple::point(-8.0, 4.0, -8.0);
    let light_color = Tuple::color(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    //////////// Sol ////////////////
    let mut floor = Plane::new();
    floor.material.pattern = Some(Pattern::Checker(Checker::new(
        Tuple::color(0.0, 0.0, 0.0),
        Tuple::color(1.0, 1.0, 1.0),
    )));
    ////////////////////////////////////////////////////////////

    //////////// Mur Gauche ////////////////
    let mut left_wall = Plane::new();
    left_wall
        .set_transform(translation(0.0, 0.0, 8.0) * rotation_y(PI / 5.0) * rotation_x(PI / 2.0));
    left_wall.material.color = Tuple::color(0.5, 0.5, 0.5);
    //////////////////////////////////////////////////////////////



    ////////////// Mur Droit ////////////////
    let mut right_wall = Plane::new();
    right_wall
        .set_transform(translation(0.0, 0.0, 8.0) * rotation_y(-PI / 5.0) * rotation_x(PI / 2.0));
    right_wall.material.color = Tuple::color(0.75, 0.75, 0.75);
    //////////////////////////////////////////////////////////////


    //////////// Cylindre ////////////////
    let mut cylindre = Cylinder::new();
    cylindre.transform = translation(-2.5, 1.5, 4.0) * scaling(1.5, 1.5, 1.5);
    cylindre.material = Material::new();
    cylindre.material.color = Tuple::color(0.5, 1.0, 0.1);
    cylindre.material.diffuse = 0.7;
    cylindre.material.specular = 0.3;
    //////////////////////////////////////////////////////////////



    ///////////// Sphere Middle //////////////
    let mut middle = Sphere::new();
    middle.transform = translation(2.0, 2.0, 2.5) * scaling(2.0, 2.0, 2.0);
    middle.material = Material::new();
    middle.material.color = Tuple::color(0.5, 1.0, 0.1);
    middle.material.reflective = 0.5;
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    ////////////////////////////////////////////////////
    
    
    //////////////// Cube ///////////////////   
    let mut cube = Cube::new();
    cube.transform = translation(0.0, 2.0, 0.0);
    cube.material.color = Tuple::color(1.0, 1.0, 1.0);
    cube.material.reflective = 0.8;
    cube.material.diffuse = 0.7;
    cube.material.specular = 0.3;
    ////////////////////////////////////////////


    //////////////// World ///////////////////
    let world = World::new(
        light.clone(),
        &[
            Shape::Plane(left_wall),
            Shape::Plane(right_wall),
            Shape::Plane(floor),
            Shape::Sphere(middle),
            Shape::Cube(cube),
            Shape::Cylinder(cylindre),
        ],
    );
    /////////////////////////////////////////////
    

    ///////////// Camera ///////////////////
    let mut camera = Camera::new(800.0, 400.0, PI / 2.0);
    // let mut camera = Camera::new(140.0, 70.0, PI / 2.0);

    camera.transform = view_transformation(
        Tuple::point(1.0, 7.0, -8.0), 
        Tuple::point(0.0, 1.0, 0.0), 
        Tuple::vector(0.0, 2.0, 0.0),
    );
    camera.render(&world).to_ppm();
}