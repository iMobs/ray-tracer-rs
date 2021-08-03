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

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn format_color(self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self.x()) as u64,
            (255.999 * self.y()) as u64,
            (255.999 * self.z()) as u64
        )
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
        Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
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
        *self = Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vec3::new(self.x() / other, self.y() / other, self.z() / other)
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
        assert_eq!(vec3.format_color(), "25 51 76");
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
    fn vec3_mul_other() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec3 * vec3, Vec3::new(1.0, 4.0, 9.0));
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
