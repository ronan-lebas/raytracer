use serde::Deserialize;

/// Represents a 3D vector, point, or RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Creates a new Vec3.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns the squared length of the vector.
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the length (magnitude) of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns a normalized version of the vector (length of 1).
    pub fn unit_vector(&self) -> Self {
        let len = self.length();
        Self::new(self.x / len, self.y / len, self.z / len)
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Reflects a vector v around a normal vector n.
    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v + (*n * (-2.0 * n.dot(v)))
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec_length() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vec_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, -5.0, 6.0);
        assert_eq!(v1.dot(&v2), 12.0); // 4 - 10 + 18
    }
}
