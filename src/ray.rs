use super::vec::{Point3, Vec3};

#[derive(Debug, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn orig(&self) -> Point3 {
        self.orig
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_new() {
        let orig = Point3::new(0.1, 0.2, 0.3);
        let dir = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(orig, dir);

        assert_eq!(ray, Ray { orig, dir })
    }

    #[test]
    fn ray_orig() {
        let orig = Point3::new(0.1, 0.2, 0.3);
        let dir = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(orig, dir);

        assert_eq!(ray.orig(), orig)
    }

    #[test]
    fn ray_dir() {
        let orig = Point3::new(0.1, 0.2, 0.3);
        let dir = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(orig, dir);

        assert_eq!(ray.dir(), dir)
    }

    #[test]
    fn ray_at() {
        let orig = Point3::new(0.1, 0.2, 0.3);
        let dir = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(orig, dir);

        assert_eq!(ray.at(1.0), Point3::new(0.2, 0.4, 0.6))
    }
}
