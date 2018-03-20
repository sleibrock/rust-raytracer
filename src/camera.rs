// camera.rs
// used to fire rays into a world

use v3::*;
use ray::*;


pub struct Camera {
    pub origin:     V3,
    pub bot_left:   V3,
    pub horizontal: V3,
    pub vertical:   V3,
}


// TODO: work on accepting an origin, FOV, depth of field, etc
impl Camera {
    pub fn new() -> Camera {
        Camera{
            origin:     V3::new( 0.0,  0.0,  0.0),
            bot_left:   V3::new(-2.0, -1.0, -1.0),
            horizontal: V3::new( 4.0,  0.0,  0.0),
            vertical:   V3::new( 0.0,  2.0,  0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray{
            position: self.origin,
            direction: self.bot_left
                + (self.horizontal*u)
                + (self.vertical*v)
                - (self.origin)
        }
    }
}


// end camera.rs
