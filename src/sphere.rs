use serde::Deserialize;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

/// Represents a sphere in 3D space.
/// Deserialized directly from our scene.json file.
#[derive(Debug, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
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

        Some(HitRecord { p, normal, t: (t_star), material: &self.material })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_deserialization() {
        let json_data_matte = r#"
        {
            "center": { "x": 1.0, "y": 2.0, "z": 3.0 },
            "radius": 4.5,
            "material": {
                "type": "Matte",
                "color": { "x": 0.0, "y": 0.0, "z": 0.0}
            }
        }"#;

        let json_data_mirror = r#"
        {
            "center": { "x": 1.0, "y": 2.0, "z": 3.0 },
            "radius": 4.5,
            "material": {
                "type": "Mirror"
            }
        }"#;

        let json_data_glass = r#"
        {
            "center": { "x": 1.0, "y": 2.0, "z": 3.0 },
            "radius": 4.5,
            "material": {
                "type": "Glass",
                "index_of_refraction": 1.0
            }
        }"#;

        let sphere_matte: Sphere = serde_json::from_str(json_data_matte).unwrap();
        assert_eq!(sphere_matte.center.x, 1.0);
        assert_eq!(sphere_matte.radius, 4.5);
        match sphere_matte.material {
            Material::Matte { color } => {
                assert_eq!(color.x, 0.0);
            },
            _ => panic!("Expected Matte material"),
        }
        let sphere_mirror: Sphere = serde_json::from_str(json_data_mirror).unwrap();
        match sphere_mirror.material {
            Material::Mirror => {},
            _ => panic!("Expected Mirror material"),
        }
        let sphere_glass: Sphere = serde_json::from_str(json_data_glass).unwrap();
        match sphere_glass.material {
            Material::Glass { index_of_refraction } => {
                assert_eq!(index_of_refraction, 1.0);
            },
            _ => panic!("Expected Glass material"),
        }
    }
}