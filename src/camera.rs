// camera.rs
// used to fire rays into a world

use std::f64::consts::PI;
use v3::*;
use ray::*;


pub struct Camera {
    pub position:    V3,
    pub target:      V3,
    pub view_up:     V3,
    pub vert_fov:   f64,
    pub aspect:     f64,
    pub aperture:   f64,
    pub focus_dist: f64,
}


// The Camera struct accepts a large number of parameters, so
// the best way would be to chain methods together instead of having
// long-line parameter definitions or such
impl Camera {
    pub fn new(pos: V3) -> Camera {
        Camera{
            position:  pos,
            target:    V3::new(1.0, 0.0, 0.0),
            view_up:   V3::new(0.0, 1.0, 0.0),
            vert_fov:  90.0,
            aspect:    1.0,
            aperture:  1.0,
            focus_dist: 1.0,
        }
    }

    pub fn position(mut self, p: V3) -> Camera {
        self.position = p; self
    }

    pub fn target(mut self, d: V3) -> Camera {
        self.target = d; self
    }

    pub fn view_up(mut self, vup: V3) -> Camera {
        self.view_up = vup; self
    }

    pub fn fov(mut self, vfov: f64) -> Camera {
        self.vert_fov = vfov; self
    }

    pub fn aspect_ratio(mut self, rat: f64) -> Camera {
        self.aspect = rat; self
    }

    pub fn aperture(mut self, ap: f64) -> Camera {
        self.aperture = ap; self
    }

    pub fn focus(mut self, fd: f64) -> Camera {
        self.focus_dist = fd; self
    }


    // calculate a ray using the camera metrics (yes it's annoying)
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let lens_radius = self.aperture * 0.5;
        let theta = self.vert_fov * (PI / 180.0); 
        let half_height = (theta/2.0).tan();
        let half_width = self.aspect * half_height;
        let w = (self.position - self.target).normal();
        let u = self.view_up.cross(&w).normal();
        let v = w.cross(&u);
        let bleft = self.position
            - (half_width * u * self.focus_dist)
            - (half_height * v * self.focus_dist)
            - (w * self.focus_dist);
        let horiz = 2.0 * half_width * u * self.focus_dist;
        let verti = 2.0 * half_height * v * self.focus_dist;
        let rd = lens_radius * random_in_unit_disk();
        let offset = u * rd.x + v * rd.y;
        Ray{
            position: self.position + offset,
            direction: bleft+(horiz*s)+(verti*t)-self.position-offset,
        }
    }
}


// end camera.rs
