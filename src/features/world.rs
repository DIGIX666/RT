use super::{
    intersections::{computations::Computation, hit, intersections, Intersection},
    lights::Light,
    materials::{lightning, Material},
    rays::Ray,
    shape::Shape,
    spheres::Sphere,
    transformations::scaling,
    tuple::Tuple,
};

pub struct World {
    pub light: Light,
    pub shapes: Vec<Shape>,
}

impl World {
    pub fn new(light: Light, shapes: &[Shape]) -> Self {
        Self {
            light,
            shapes: shapes.to_vec(),
        }
    }
    pub fn shade_hit(&self, comps: &Computation) -> Tuple {
        let shadowed = self.is_shadowed(&comps.over_point);
        let surface = lightning(
            &comps.object.material(),
            &comps.object,
            &self.light,
            &comps.over_point,
            &comps.eyev,
            &comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(comps);
        surface + reflected
    }
    pub fn color_at(&self, r: &Ray) -> Tuple {
        if let Some(i) = hit(intersect_world(self, r)) {
            let comps = Computation::new(&i, r, &intersect_world(self, r));
            return self.shade_hit(&comps);
        }
        Tuple::default_color()
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        let intersections = intersect_world(self, &r);
        if let Some(h) = hit(intersections) {
            if h.t < distance {
                return true;
            }
        }
        false
    }
    pub fn reflected_color(&self, comps: &Computation) -> Tuple {
        if comps.object.material().reflective == 0.0 {
            return Tuple::default_color();
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray);
        color * comps.object.material().reflective
    }
}

pub fn intersect_world(world: &World, ray: &Ray) -> Vec<Intersection> {
    let mut out = vec![];
    for ix in 0..world.shapes.len() {
        let mut xs = world.shapes[ix].intersect(ray);
        out.append(&mut xs);
    }
    intersections(&mut out)
}
impl Default for World {
    fn default() -> Self {
        let light = Light::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Tuple::color(1.0, 1.0, 1.0),
        );
        let mut s1 = Shape::Sphere(Sphere::new());
        let mut material = Material::new();
        material.color = Tuple::color(0.8, 1.0, 0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        s1.set_material(material);
        let mut s2 = Shape::Sphere(Sphere::new());
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        let shapes = &[s1, s2];
        Self::new(light, shapes)
    }
}