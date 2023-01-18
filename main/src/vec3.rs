use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    //coordinates
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    //default vector
    pub fn default_vec() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    //vector with specified coordinates
    pub fn vec(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    //vector with all coordinates as the same value
    pub fn vec_all(all: f32) -> Vec3 {
        Vec3 {
            x: all,
            y: all,
            z: all,
        }
    }

    //returns the length of the vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    //returns a normalised vector
    pub fn normalised_vector(&self) -> Vec3 {
        Vec3 {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    //returns the dot product of two vectors
    pub fn dot_product(u: Vec3, v: Vec3) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    //returns the cross product of two vectors
    pub fn cross_product(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
}

//implementing the add, subtract, multiply, divide and change of sign operators
impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super ::*;

    #[test]
    fn test_vec3_add() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = Vec3::vec(4.0, 5.0, 6.0);
        let c = a + b;
        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 7.0);
        assert_eq!(c.z, 9.0);
    }

    #[test]
    fn test_vec3_sub() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = Vec3::vec(4.0, 5.0, 6.0);
        let c = a - b;
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, -3.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn test_vec3_mul() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = a * 2.0;
        assert_eq!(b.x, 2.0);
        assert_eq!(b.y, 4.0);
        assert_eq!(b.z, 6.0);
    }

    #[test]
    fn test_vec3_div() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = a / 2.0;
        assert_eq!(b.x, 0.5);
        assert_eq!(b.y, 1.0);
        assert_eq!(b.z, 1.5);
    }

    #[test]
    fn test_vec3_neg() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = -a;
        assert_eq!(b.x, -1.0);
        assert_eq!(b.y, -2.0);
        assert_eq!(b.z, -3.0);
    }

    #[test]
    fn test_vec3_length() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        assert_eq!(a.length(), 3.7416575);
    }

    #[test]
    fn test_vec3_normalised_vector() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = a.normalised_vector();
        assert_eq!(b.x, 0.26726124);
        assert_eq!(b.y, 0.5345225);
        assert_eq!(b.z, 0.8017837);
    }

    #[test]
    fn test_vec3_dot_product() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = Vec3::vec(4.0, 5.0, 6.0);
        assert_eq!(Vec3::dot_product(a, b), 32.0);
    } 

    #[test]
    fn test_vec3_cross_product() {
        let a = Vec3::vec(1.0, 2.0, 3.0);
        let b = Vec3::vec(4.0, 5.0, 6.0);
        let c = Vec3::cross_product(a, b);
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, 6.0);
        assert_eq!(c.z, -3.0);
    }
}
