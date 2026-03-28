use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

/// Records the details of a successful ray intersection.
/// Contains the point of contact, the normal vector at that point,
/// the `t` value along the ray where the hit occurred, and the color of the object.
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'a Material,
}

/// A trait for any object that can be hit by a ray.
pub trait Hittable {
    /// Determines if a ray intersects with this object.
    /// 
    /// # Arguments
    /// * `ray` - The ray being cast.
    /// * `t_min` - The minimum distance to check (prevents hitting objects behind the camera).
    /// * `t_max` - The maximum distance to check.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}