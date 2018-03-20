// element.rs

use ray::*;
use sphere::*;
use intersectable::*;
use material::*;

pub enum Element {
    Sphere(Sphere),
}




// object initializers to return elements
pub fn new_sphere(x: f64, y: f64, z: f64, r: f64, m: Material) -> Element {
    Element::Sphere(Sphere::new(x, y, z, r, m))
}

impl Intersectable for Element {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Intersection {
        match *self {
            Element::Sphere(ref s) => s.intersect(r, t_min, t_max),
        }
    } 
}


// end element.rs
