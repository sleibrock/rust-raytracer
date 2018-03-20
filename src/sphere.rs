// sphere.rs

use v3::*;
use ray::*;
use intersectable::*;
use material::*;

pub struct Sphere {
    pub center:  V3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64, m: Material) -> Sphere {
        Sphere{ center: V3::new(x, y, z), radius: r, material: m }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Intersection {
        let oc   = ray.position - self.center;
        let a    = ray.direction * ray.direction;
        let b    = oc * ray.direction;
        let c    = oc*oc  - self.radius*self.radius;
        let disc = b*b - a*c;

        if disc > 0.0 {
            let disc_sqrt = disc.sqrt();
            let t1 = (-b - disc_sqrt)/a;
            if t1 < t_max && t1 > t_min {
                return Intersection::Hit(
                    t1,
                    ray.point_at(t1),
                    (ray.point_at(t1) - self.center) / self.radius,
                    self.material,
                );
            }
            let t2 = (-b + disc_sqrt)/a;
            if t2 < t_max && t2 > t_min {
                return Intersection::Hit(
                    t2,
                    ray.point_at(t2),
                    (ray.point_at(t2) - self.center) / self.radius,
                    self.material,
                );
            }
        }
        Intersection::None
    }
}

// end sphere.rs
