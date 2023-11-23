# Ray Tracing Project

## Introduction
This Ray Tracing project is designed to implement the ray tracing technique, enabling 3D scenes to be transformed into 2D images. It offers an in-depth understanding of image rendering principles and develops skills in computer-generated imagery (CGI), algorithms, geometry and mathematics.

## Features
- **Objects supported**: Sphere, cube, flat plane, cylinder.
- **Dynamic modification**: Change object location before image creation.
- **Flexible camera view**: View scene from different angles.
- **Light management** : Variable brightness and shadows.

## Ray Tracing
### Creat Elements
```rust
// in draw.rs, line 26

    let mut floor = Plane::new();
    floor.material.pattern = Some(Pattern::Checker(Checker::new(
        Tuple::color(0.0, 0.0, 0.0),
        Tuple::color(1.0, 1.0, 1.0),
    )));
```
But before, you want create file for create structure your element. 
### Example
```rust
use uuid::Uuid;
use crate::features::shape::Shape;
use crate::materials::Material;
use crate::matrice::Matrice;
use crate::rays::Ray;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Cube {
    pub id: Uuid,
    pub transform: Matrice,
    pub material: Material,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: Matrice::identity_matrix(4),
            material: Material::new(),
        }
    }

    // Méthode pour calculer les intersections entre le cube et un rayon
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {}

    // Méthode pour obtenir la normale à un point sur la surface du cube
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        // Calculez la normale en fonction de la position du point par rapport au cube
        Tuple::vector(1.0, 0.0, 0.0) // Retourne un vecteur
    }
}

// Vous pouvez ensuite créer et configurer un cube comme ceci :
let mut cube = Cube::new();
cube.transform = Matrice::identity_matrix(4); // Appliquer les transformations nécessaires
cube.material = Material::new(); // Configurer le matériau
// Configurez d'autres propriétés du cube selon les besoins
```

### Modify light
```rust
// in draw.rs, line 22

    let light_color = Tuple::color(1.0, 1.0, 1.0); // change color rgb 
```
### Move Camera
```rust
// in draw.rs, line 101

    camera.transform = view_transformation(
            Tuple::point(1.0, 7.0, -8.0), // Position of camera
            Tuple::point(0.0, 1.0, 0.0), // Orientation of camera
            Tuple::vector(0.0, 2.0, 0.0), // Orientation Vertical of camera
        );
```

## Run Project
### Uploading Image
```
cargo run > image.ppm 
```

### Open Image
This project uses the PPM image format, which can be opened with the following software:
- [GIMP](https://www.gimp.org/) 