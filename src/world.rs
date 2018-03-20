// world.rs

use v3::*;
use ray::*;
use element::*; 
use intersectable::*;


pub struct World {
    pub objects: Vec<Element>,
}


impl World {
    pub fn new() -> World {
        let v : Vec<Element> = Vec::new();
        World{ objects: v }
    }

    pub fn push(&mut self, obj: Element) {
        self.objects.push(obj);
    }
}


// inject a ray into the World to get the closest hit
impl Intersectable for World {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Intersection {
        let mut closest = t_max;
        let mut intersected = Intersection::None;
        for obj in &self.objects {

            match obj.intersect(r, t_min, closest) {
                Intersection::None => {}
                Intersection::Hit(t, pos, nrm, mat) => {
                    if t < closest {
                        closest = t;
                        intersected = Intersection::Hit(t, pos, nrm, mat);
                    }
                }
            }
        }
        intersected
    }
}

// end world.rs
