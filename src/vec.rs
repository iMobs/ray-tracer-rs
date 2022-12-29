use rand::prelude::*;
use std::ops::Range;

pub use glam::DVec3 as Vec3;

pub trait Vec3Ext {
    fn random(r: Range<f64>) -> Self;
    fn random_in_unit_sphere() -> Self;
    fn random_in_hemisphere(normal: Self) -> Self;
    fn random_in_unit_disk() -> Self;
    fn reflect(self, rhs: Self) -> Self;
    fn refract(self, rhs: Self, etai_over_etat: f64) -> Self;
    fn format_color(self, samples_per_pixel: u64) -> String;
}

impl Vec3Ext for Vec3 {
    fn random(r: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
            rng.gen_range(r),
        )
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

    fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let p = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    fn reflect(self, n: Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    fn refract(self, rhs: Self, etai_over_etat: f64) -> Self {
        let cos_theta = ((-1.0) * self).dot(rhs).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * rhs);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * rhs;
        r_out_perp + r_out_parallel
    }

    fn format_color(mut self, samples_per_pixel: u64) -> String {
        self /= samples_per_pixel as f64;

        let ir = (256.0 * self.x.sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * self.y.sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * self.z.sqrt().clamp(0.0, 0.999)) as u64;

        format!("{ir} {ig} {ib}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_dot() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3.dot(vec3), 0.14);
    }

    #[test]
    fn vec3_length() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3.length(), 0.37416573867739417);
    }

    #[test]
    fn vec3_cross() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(
            vec3.cross(Vec3::new(0.3, 0.2, 0.1)),
            Vec3::new(
                -0.039999999999999994,
                0.07999999999999999,
                -0.039999999999999994
            )
        );
    }

    #[test]
    fn vec3_normalized() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(
            vec3.normalize(),
            Vec3::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372731)
        );
    }

    #[test]
    fn vec3_format_color() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3.format_color(2), "57 80 99");
    }
}
