use super::vec::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_new() {
        let origin = Vec3::new(0.1, 0.2, 0.3);
        let direction = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray, Ray { origin, direction })
    }

    #[test]
    fn ray_orig() {
        let orig = Vec3::new(0.1, 0.2, 0.3);
        let dir = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(orig, dir);

        assert_eq!(ray.origin(), orig)
    }

    #[test]
    fn ray_dir() {
        let orig = Vec3::new(0.1, 0.2, 0.3);
        let dir = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(orig, dir);

        assert_eq!(ray.direction(), dir)
    }

    #[test]
    fn ray_at() {
        let orig = Vec3::new(0.1, 0.2, 0.3);
        let dir = Vec3::new(0.1, 0.2, 0.3);
        let ray = Ray::new(orig, dir);

        assert_eq!(ray.at(1.0), Vec3::new(0.2, 0.4, 0.6))
    }
}
