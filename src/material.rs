// material.rs

use v3::*;
use ray::*;
use intersectable::*;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambert(V3),
    Metal(V3),
    Glass(V3),
}

pub enum Incident {
    None,
    Scattered(V3, Ray),
}


impl Material {
    pub fn scatter(&self, r: &Ray, hit: Intersection) -> Incident {
        match *self {
            Material::Lambert(lv) => calc_lambert(lv, r, hit),
            Material::Metal(mv)   => calc_metal(mv, r, hit),
            Material::Glass(gv)   => calc_glass(gv, r, hit),
        }
    }  
}

// short initializers
pub fn lambert(x: f64, y: f64, z: f64) -> Material {
    Material::Lambert(V3::new(x, y, z))
}
pub fn metal(x: f64, y: f64, z: f64) -> Material {
    Material::Metal(V3::new(x, y, z))
}
pub fn glass(x: f64, y: f64, z: f64) -> Material {
    Material::Glass(V3::new(x, y, z))
}


// define interactions here
pub fn calc_lambert(albedo: V3, r: &Ray, hit: Intersection) -> Incident {
    return match hit {
        Intersection::Hit(t, p, nrm, _) => {
            let target = p + nrm + random_in_unit_sphere();
            Incident::Scattered(
                albedo,
                Ray{ position: p, direction: target - p }
            )
        }
        Intersection::None => Incident::None,
    };
}

pub fn calc_metal(albedo: V3, r: &Ray, hit: Intersection) -> Incident {
    return match hit {
        Intersection::Hit(t, p, nrm, _) => {
            let refl = r.direction.reflect(&nrm);
            if refl*nrm > 0.0 {
                Incident::Scattered(
                    albedo,
                    Ray{ position: p, direction: refl }
                )
            } else {
                Incident::None
            }
        }
        Intersection::None => Incident::None,
    };
}

pub fn calc_glass(albedo: V3, r: &Ray, hit: Intersection) -> Incident {
    Incident::None
}


// end material.rs
