// plane.rs

use v3::*;
use ray::*;
use intersectable::*;
use material::*;

pub struct Plane {
    pub position: V3,
    pub normal:   V3,
    pub material: Material,
}


impl Plane {
    pub fn new(
        x: f64, y: f64, z: f64,
        dx: f64, dy: f64, dz: f64,
        m: Material
    ) -> Plane {
        Plane {
            position: V3::new(x, y, z),
            normal: V3::new(dx, dy, dz),
            material: m
        }
    }
}


// (ray_origin + (ray_direction*t) - plane_origin) * plane_normal = 0
// t = (plane_origin - ray_origin) * normal / ray_direction * normal
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Intersection {
        let denom = ray.direction * self.normal;
        if denom > 0.0 {
            let t = ((self.position  - ray.position) * self.normal) / denom;

            if t < t_max && t > t_min {
                return Intersection::Hit(
                    t,
                    ray.point_at(t),
                    -self.normal,
                    self.material
                );
            }
        }
        Intersection::None
    }
}


// end plane.rs
