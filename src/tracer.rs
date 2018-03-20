// tracer.rs


use v3::*;
use world::*;
use ray::*;
use render::*;
use intersectable::*;
use material::*;

pub fn trace(r: &Ray, w: &World, limit: u64) -> V3 {
    let intersect = w.intersect(r, 0.001, 25000.0);
    match intersect {
        // object was detected, calculate the surface normal color
        Intersection::Hit(t, p, nrm, mat) => {
            if limit != 0 {
                match mat.scatter(r, intersect) {
                    Incident::Scattered(atten, scattered) => {
                        return trace(&scattered, w, limit-1).product(&atten);
                    },
                    Incident::None  => {
                        return V3::zeroes();
                    },
                }
            }
            return V3::zeroes();
        }
        
        // nothing hit, use a background color
        _ => {
            let ud = r.direction.normal();
            let t = 0.5*(ud.y + 1.0);
            return (1.0-t)*V3::ones() + t*V3::new(0.5, 0.7, 1.0);
        }
    }

}


// end tracer.rs
