use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0 * (self[0] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] / (samples_per_pixel as f64)).clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib,)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.x(), self.y(), self.z())
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
        Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self[0] * other, self[1] * other, self[2] * other)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3::new(self[0] * other, self[1] * other, self[2] * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other[0], self * other[1], self * other[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self[0] / other, self[1] / other, self[2] / other)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3::new(self[0] / other, self[1] / other, self[2] / other)
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
        assert_eq!(vec3.format_color(2), "12 25 38");
    }

    #[test]
    fn vec3_display() {
        let vec3 = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(format!("{}", vec3), "(0.1 0.2 0.3)")
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
