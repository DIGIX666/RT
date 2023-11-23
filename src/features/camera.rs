pub mod cameras {
    use indicatif::ProgressBar;

    use crate::features::{
        canvas::Canvas, matrice::Matrice, rays::Ray, tuple::Tuple, world::World,
    };

    pub struct Camera {
        pub hsize: f32,
        pub vsize: f32,
        pub field_of_view: f32,
        pub transform: Matrice,
        pub pixel_size: f32,
        pub half_width: f32,
        pub half_height: f32,
    }

    impl Camera {
        pub fn new(hsize: f32, vsize: f32, field_of_view: f32) -> Camera {
            let mut out = Self {
                hsize,
                vsize,
                field_of_view,
                transform: Matrice::identity_matrix(4),
                pixel_size: 0.0,
                half_height: 0.0,
                half_width: 0.0,
            };
            let half_view = (field_of_view / 2.0).tan();
            let aspect = hsize / vsize;
            if aspect >= 1.0 {
                out.half_width = half_view;
                out.half_height = half_view / aspect;
            } else {
                out.half_width = half_view * aspect;
                out.half_height = half_view;
            }
            out.pixel_size = (out.half_width * 2.0) / out.hsize;
            out
        }
        pub fn ray_for_pixel(&self, px: f32, py: f32) -> Ray {
            // the offset from the edge of the canvas to the pixel's center
            let xoffset = (px + 0.5) * self.pixel_size;
            let yoffset = (py + 0.5) * self.pixel_size;

            //  the untransformed coordinates of the pixel in world space.
            // (remember that the camera looks toward -z, so +x is to the *left*.)
            let world_x = self.half_width - xoffset;
            let world_y = self.half_height - yoffset;

            //  using the camera matrix, transform the canvas point and the origin,
            //  and then compute the ray's direction vector.
            //  (remember that the canvas is at z=-1)

            let pixel = self.transform.inverse().unwrap() * Tuple::point(world_x, world_y, -1.0);
            let origin = self.transform.inverse().unwrap() * Tuple::point(0.0, 0.0, 0.0);
            let direction = (pixel - origin).normalize();
            Ray::new(origin, direction)
        }
        pub fn render(&self, world: &World) -> Canvas {
            let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
            let bar = ProgressBar::new(self.vsize as u64);
            for y in 0..self.vsize as usize - 1 {
                for x in 0..self.hsize as usize - 1 {
                    let ray = self.ray_for_pixel(x as f32, y as f32);
                    let color = world.color_at(&ray);
                    image.write_pixel(x, y, color);
                }
                bar.inc(1);
            }
            image
        }
    }
}