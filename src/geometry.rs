// geometric objects and definitions

use vector::*;
use shading::*;


// A ray is a line starting from a position shooting "infinitely"
// The direction is a Vector in which the ray is traveling [forever]
pub struct Ray {
    pub    pos: V3,
    pub    dir: V3,
}



// not there yet...
// A plane is an "infinite" span, the position represents the center
// The normal, or "surface" normal represents the direction of the plane
pub struct Plane {
    pub    pos:       V3,
    pub normal:       V3,
    pub    mat: Material,
}


// A Sphere is a spherical object with a position and a fixed radius
pub struct Sphere {
    pub    pos:       V3,
    pub radius:      f64,
    pub    mat: Material,
}



// Implement Ray methods for raytracing
// Since the ray is the most fundamental object for testing collisions,
impl Ray {


    // check if a ray will collide with a sphere
    pub fn hits_sphere(&self, s: &Sphere) -> bool {
        let a = self.dir * self.dir;
        let dist = self.pos - s.pos;
        let b = 2.0 * (self.dir * dist);
        let c = (dist*dist) - (s.radius * s.radius);
        let disc = (b*b) - (4.0 * a * c);

        if disc < 0.0 {
            return false;
        } else {
            return true;
        }
    }

}
