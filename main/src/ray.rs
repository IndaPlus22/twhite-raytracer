use crate::vec3::Vec3;

pub struct Ray {
    pub u: Vec3,
    pub v: Vec3,
}

impl Ray {
    pub fn create_ray(u: Vec3, v: Vec3) -> Ray {
        Ray { u: u, v: v }
    }

    pub fn origin(&self) -> Vec3 {
        self.u
    }

    pub fn direction(&self) -> Vec3 {
        self.v
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.u + self.v * t
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ray_create_ray() {
        let u = Vec3::vec(1.0, 2.0, 3.0);
        let v = Vec3::vec(4.0, 5.0, 6.0);
        let r = Ray::create_ray(u, v);
        assert_eq!(r.u, u);
        assert_eq!(r.v, v);
    }

    #[test]
    fn test_ray_origin() {
        let u = Vec3::vec(1.0, 2.0, 3.0);
        let v = Vec3::vec(4.0, 5.0, 6.0);
        let r = Ray::create_ray(u, v);
        assert_eq!(r.origin(), u);
    }

    #[test]
    fn test_ray_direction() {
        let u = Vec3::vec(1.0, 2.0, 3.0);
        let v = Vec3::vec(4.0, 5.0, 6.0);
        let r = Ray::create_ray(u, v);
        assert_eq!(r.direction(), v);
    }

    #[test]
    fn test_ray_point_at_parameter() {
        let u = Vec3::vec(1.0, 2.0, 3.0);
        let v = Vec3::vec(4.0, 5.0, 6.0);
        let r = Ray::create_ray(u, v);
        assert_eq!(r.point_at_parameter(0.0), u);
        assert_eq!(r.point_at_parameter(1.0), u + v);
        assert_eq!(r.point_at_parameter(2.0), u + v * 2.0);
    }
}