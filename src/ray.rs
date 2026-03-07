use crate::vec3::Vec3;

/// Represents a ray shot into the scene.
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new ray.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Calculates the position of the ray at parameter `t`.
    /// 
    /// Formula: P(t) = A + tb (where A is origin, b is direction)
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let r = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(r.at(2.0), Vec3::new(1.0, 3.0, 1.0));
    }
}
