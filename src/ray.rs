// ray.rs

use v3::*;


pub struct Ray {
    pub position:  V3,
    pub direction: V3,
}


impl Ray {
    pub fn new(x1: f64, y1: f64, z1: f64,
               x2: f64, y2: f64, z2: f64) -> Ray {
        Ray{
            position: V3::new(x1, y1, z1),
            direction: V3::new(x2, y2, z2)
        }

    }

    // Origin + (Direction*t)
    pub fn point_at(&self, t: f64) -> V3 {
        self.position + self.direction * t
    }
}

// end ray.rs
