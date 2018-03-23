// material.rs

use v3::*;
use ray::*;
use utils::*;
use intersectable::*;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambert(V3),
    Metal(V3, f64),
    Glass(f64),
}

pub enum Incident {
    None,
    Scattered(V3, Ray),
    Refracted(V3),
}


// refract a ray
pub fn refract(v: V3, n: V3, ni_nt: f64) -> Incident {
    let uv = v.normal();
    let dt = uv*n;
    let disc = 1.0 - (ni_nt*ni_nt)*(1.0 - dt*dt);
    match disc > 0.0 {
        true => Incident::Refracted(ni_nt*(uv-(n*dt)) - n*disc.sqrt()),
        _ => Incident::None,
    }
}


// define the Schlick equation for Fresnel refractions
pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r1 = r0*r0;
    r1 + (1.0 - r1) * (1.0 - cosine).powi(5)
}



impl Material {
    pub fn scatter(&self, r: &Ray, hit: Intersection) -> Incident {
        match *self {
            Material::Lambert(lv)     => calc_lambert(lv, r, hit),
            Material::Metal(mv, fuzz) => calc_metal(mv, fuzz, r, hit),
            Material::Glass(gv)       => calc_glass(gv, r, hit),
        }
    }  
}

// short initializers
pub fn lambert(x: f64, y: f64, z: f64) -> Material {
    Material::Lambert(V3::new(x, y, z))
}
pub fn metal(x: f64, y: f64, z: f64, fuzz: f64) -> Material {
    Material::Metal(V3::new(x, y, z), match fuzz < 1.0 {
        true => fuzz,
        _    => 1.0,
    })
}
pub fn glass(ref_idx: f64) -> Material {
    Material::Glass(ref_idx)
}


// define interactions here
pub fn calc_lambert(albedo: V3, r: &Ray, hit: Intersection) -> Incident {
    match hit {
        Intersection::Hit(t, p, nrm, _) => {
            let target = p + nrm + random_in_unit_sphere();
            Incident::Scattered(
                albedo,
                Ray{ position: p, direction: target - p }
            )
        }
        _ => Incident::None,
    }
}

pub fn calc_metal(albedo: V3, fuz: f64, r: &Ray, hit: Intersection) -> Incident {
    match hit {
        Intersection::Hit(t, p, nrm, _) => {
            let refl = r.direction.reflect(&nrm);
            if refl*nrm > 0.0 {
                Incident::Scattered(
                    albedo,
                    Ray{
                        position: p,
                        direction: refl+(fuz*random_in_unit_sphere())
                    }
                )
            } else {
                Incident::None
            }
        }
        _ => Incident::None,
    }
}

pub fn calc_glass(refrac: f64, r: &Ray, hit: Intersection) -> Incident {
    
    match hit {
        Intersection::Hit(t, p, nrm, _) => {
            let reflected = r.direction.reflect(&nrm);
            let atten = V3::ones();
            
            let mut outward_normal = V3::zeroes();
            let mut ni_over_nt = 0.0;
            let mut refl_prob = 0.0;
            let mut cosine = 0.0;
            let mut refracted = V3::zeroes();
            let mut scray = Ray::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            
            if r.direction*nrm > 0.0 {
                outward_normal = -nrm;
                ni_over_nt = refrac;
                cosine = (refrac*r.direction*nrm)/r.direction.length();
            } else {
                outward_normal = nrm;
                ni_over_nt = 1.0 / refrac;
                cosine = -(refrac*r.direction*nrm)/r.direction.length();
            }

            match refract(r.direction, outward_normal, ni_over_nt) {
                Incident::Refracted(rr) => {
                    refl_prob = schlick(cosine, refrac);
                    refracted = rr;
                }
                _ => {
                    scray = Ray{ position: p, direction: reflected };
                    refl_prob = 1.0;
                }
            }

            if random() < refl_prob {
                scray = Ray{ position: p, direction: reflected };
            } else {
                scray = Ray{ position: p, direction: refracted };
            }
            
            Incident::Scattered(atten, scray)
        },
        
        _ => Incident::None,
    }
}


// end material.rs
