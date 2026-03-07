use serde::Deserialize;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

/// Represents a sphere in 3D space.
/// Deserialized directly from our scene.json file.
#[derive(Debug, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Vec3,
}

impl Hittable for Sphere {
    /// Calculates if a ray intersects with this sphere.
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        // Quadratic equation definition:
        let delta: Vec3 = _ray.origin - self.center;
        let a: f64 = _ray.direction.dot(&_ray.direction);
        let b: f64 = 2.0 * _ray.direction.dot(&delta);
        let c: f64 = - self.radius.powf(2.0) + delta.dot(&delta);
        
        // Cases according to the sign of the discriminant:
        let discriminant: f64 = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Quadratic equation solutions:
        let t1: f64 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2: f64 = (-b + discriminant.sqrt()) / (2.0 * a);

        // Select nearest feasible solution
        if t2 < _t_min {
            return None;
        }

        if t1 > _t_max {
            return None;
        }

        // There is a solution, call it t_star.
        let t_star: f64;
        if t2 > _t_max {
            t_star = t1;
        }

        else if t1 < _t_min {
            t_star = t2;
        }

        else {
            t_star = f64::min(t1, t2);
        }

        let p: Vec3 = _ray.at(t_star);
        let normal: Vec3 = (p - self.center).unit_vector();

        Some(HitRecord { p, normal, t: (t_star), color: self.color })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_deserialization() {
        let json_data = r#"
        {
            "center": { "x": 1.0, "y": 2.0, "z": 3.0 },
            "radius": 4.5,
            "color": { "x": 0.0, "y": 0.0, "z": 0.0}
        }"#;

        let sphere: Sphere = serde_json::from_str(json_data).unwrap();
        assert_eq!(sphere.center.x, 1.0);
        assert_eq!(sphere.radius, 4.5);
    }
}