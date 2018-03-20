// intersectable.rs

use v3::*;
use ray::*;
use material::*;


// when defining intersection returns, use this Enumeration
// None is for when no collision is found, Hit is for the "closest" hit
pub enum Intersection {
    None,
    Hit(f64, V3, V3, Material),
}


pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Intersection;
}


// end intersectable.rs
