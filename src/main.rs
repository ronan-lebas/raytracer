mod vec3;
mod ray;
mod hittable;
mod sphere;
mod material;

use vec3::Vec3;
use ray::Ray;
use hittable::Hittable;
use sphere::Sphere;
use material::Material;
use serde::Deserialize;
use core::f64;
use std::fs::File;
use std::io::{Write, BufWriter, BufReader};

#[derive(Deserialize)]
struct Light {
    position: Vec3,
}

/// Represents our entire 3D world containing all objects.
#[derive(Deserialize)]
struct World {
    spheres: Vec<Sphere>,
    light: Light,
}

impl Hittable for World {
    /// Iterates through all objects in the world and finds the closest hit.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for sphere in &self.spheres {
            if let Some(rec) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}

/// Calculates the color for a given ray.
/// Checks against the world to see if it hits any objects.
fn ray_color(ray: &Ray, world: &World) -> Vec3 {

    let eps: f64 = 0.001;

    // Check if we hit anything in the world. 
    if let Some(_rec) = world.hit(ray, eps, f64::INFINITY) {

        // Extract components of the HitRecord:
        let p: Vec3 = _rec.p;
        let normal: Vec3 = _rec.normal;
        let object_material: &Material = _rec.material;

        return match object_material {
            &Material::Matte {color} => {
                // Shoot a new ray from the impact point going toward the light source.
                let origin: Vec3 = p + normal * eps;
                let direction: Vec3 = world.light.position - origin;
                let distance_to_light: f64 = direction.length();
                let direction_unit: Vec3 = direction.unit_vector();
                let second_ray: Ray = Ray::new(origin, direction_unit);
                if let Some(_second_rec) = world.hit(&second_ray, eps, distance_to_light) {
                    return Vec3::new(0.0, 0.0, 0.0);
                }
                color * f64::max(0.0, normal.dot(&direction_unit))
            }
            &Material::Glass { index_of_refraction } => Vec3::new(0.0, 0.0, 0.0), //TODO
            &Material::Mirror => {
                let new_ray: Ray = Ray::new(p, Vec3::reflect(&ray.direction, &normal));
                ray_color(&new_ray, world)
            }
        }
    }

    // If we hit nothing, draw the sky background
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load the scene from JSON
    let file = File::open("scene.json")?;
    let reader = BufReader::new(file);
    let world: World = serde_json::from_reader(reader)?;
    println!("Loaded {} spheres from scene.json", world.spheres.len());

    // 2. Image settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // 3. Camera setup
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal * 0.5) - (vertical * 0.5) - Vec3::new(0.0, 0.0, focal_length);

    // 4. Prepare the output file
    let output_file = File::create("output.ppm")?;
    let mut writer = BufWriter::new(output_file);
    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    // 5. Render loop
    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {} ", j);
        std::io::Write::flush(&mut std::io::stdout())?;
        
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let ray_direction = lower_left_corner + (horizontal * u) + (vertical * v) - origin;
            let ray = Ray::new(origin, ray_direction);

            let pixel_color = ray_color(&ray, &world);

            let ir = (255.999 * pixel_color.x) as i32;
            let ig = (255.999 * pixel_color.y) as i32;
            let ib = (255.999 * pixel_color.z) as i32;

            writeln!(writer, "{} {} {}", ir, ig, ib)?;
        }
    }
    
    println!("\nDone!");
    Ok(())
}