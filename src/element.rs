// element.rs

use ray::*;
use sphere::*;
use plane::*;
use intersectable::*;
use material::*;

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}




// object initializers to return elements
pub fn new_sphere(x: f64, y: f64, z: f64, r: f64, m: Material) -> Element {
    Element::Sphere(Sphere::new(x, y, z, r, m))
}

pub fn new_plane(
    x: f64, y: f64, z: f64,
    dx: f64, dy: f64, dz: f64,
    m: Material
) -> Element {
    Element::Plane(Plane::new(x, y, z, dx, dy, dz, m))
}

impl Intersectable for Element {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Intersection {
        match *self {
            Element::Sphere(ref s) => s.intersect(r, t_min, t_max),
            Element::Plane(ref p) => p.intersect(r, t_min, t_max),
        }
    } 
}


// end element.rs
