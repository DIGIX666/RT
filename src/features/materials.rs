use super::{lights::Light, patterns::Pattern, shape::Shape, tuple::Tuple};

#[derive(PartialEq, Debug, Clone)]
pub struct Material {
    pub pattern: Option<Pattern>,
    pub color: Tuple,
    pub ambient: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub reflective: f32,
}

impl Eq for Material {}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Tuple::color(1.0, 1.0, 1.0),
            pattern: None,
            transparency: 0.0,
            refractive_index: 1.0,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
        }
    }
}

pub fn lightning(
    material: &Material,
    shape: &Shape,
    light: &Light,
    position: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
) -> Tuple {
    let mut color = material.color;
    if let Some(c) = &material.pattern {
        color = c.at_object(shape, position);
    }
    let effective_color = color * light.intensity;
    let lightv = (light.position - *position).normalize();
    let ambient = effective_color * material.ambient;
    if in_shadow {
        return ambient;
    }
    let light_dot_normal = lightv.dot(normalv);
    let mut diffuse = Tuple::default_color();
    let mut specular = Tuple::default_color();

    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    ambient + diffuse + specular
}