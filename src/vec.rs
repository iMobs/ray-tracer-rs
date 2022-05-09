use rand::prelude::*;
use std::fmt;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Sub, SubAssign,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
            rng.gen_range(r),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    fn x_mut(&mut self) -> &mut f64 {
        &mut self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    fn y_mut(&mut self) -> &mut f64 {
        &mut self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    fn z_mut(&mut self) -> &mut f64 {
        &mut self[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-1.0) * self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.x().abs() < EPS && self.y().abs() < EPS && self.z().abs() < EPS
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0
            * (self.x() / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let ig = (256.0
            * (self.y() / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let ib = (256.0
            * (self.z() / (samples_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;

        format!("{ir} {ig} {ib}")
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        write!(f, "({x}, {y}, {z})")
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self.x_mut() += other.x();
        *self.y_mut() += other.y();
        *self.z_mut() += other.z();
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self.x_mut() -= other.x();
        *self.y_mut() -= other.y();
        *self.z_mut() -= other.z();
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self.x_mut() *= other;
        *self.y_mut() *= other;
        *self.z_mut() *= other;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self.x_mut() *= other.x();
        *self.y_mut() *= other.y();
        *self.z_mut() *= other.z();
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x(), self * other.y(), self * other.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self[2] / other)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self.x_mut() /= other;
        *self.y_mut() /= other;
        *self.z_mut() /= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vec3() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3, Vec3 { e: [0.1, 0.2, 0.3] });
    }

    #[test]
    fn vec3_x() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3.x(), 0.1);
    }

    #[test]
    fn vec3_y() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3.y(), 0.2);
    }

    #[test]
    fn vec3_z() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3.z(), 0.3);
    }

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
            vec3.normalized(),
            Vec3::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372731)
        );
    }

    #[test]
    fn vec3_format_color() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3.format_color(2), "57 80 99");
    }

    #[test]
    fn vec3_display() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(format!("{vec3}"), "(0.1, 0.2, 0.3)")
    }

    #[test]
    fn vec3_add() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3 + vec3, Vec3::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn vec3_add_assign() {
        let mut vec3 = Vec3::new(0.1, 0.2, 0.3);
        vec3 += vec3;
        assert_eq!(vec3, Vec3::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn vec3_sub() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3 - vec3, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_sub_assign() {
        let mut vec3 = Vec3::new(0.1, 0.2, 0.3);
        vec3 -= vec3;
        assert_eq!(vec3, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3_mul() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3 * 2.0, Vec3::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn vec3_mul_assign() {
        let mut vec3 = Vec3::new(0.1, 0.2, 0.3);
        vec3 *= 2.0;
        assert_eq!(vec3, Vec3::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn vec3_mul_reverse() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(2.0 * vec3, Vec3::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn vec3_div() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(vec3 / 2.0, Vec3::new(0.05, 0.1, 0.15));
    }

    #[test]
    fn vec3_div_assign() {
        let mut vec3 = Vec3::new(0.1, 0.2, 0.3);
        vec3 /= 2.0;
        assert_eq!(vec3, Vec3::new(0.05, 0.1, 0.15));
    }
}
