use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point3, focus: Point3, v_up: Vec3, v_fov: f64, aspect_ratio: f64) -> Camera {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * v_fov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (origin - focus).normalized();
        let cu = v_up.cross(cw).normalized();
        let cv = cw.cross(cu);

        let horizontal = viewport_width * cu;
        let vertical = viewport_height * cv;

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - cw;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
