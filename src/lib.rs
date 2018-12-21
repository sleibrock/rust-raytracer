// lib.rs
// add your rust files here
// ie: camera.rs -> pub mod camera;

/*
pub mod v3;
pub mod ray;
pub mod camera;
pub mod sphere;
pub mod plane;
pub mod element;
pub mod world;
pub mod material;
pub mod intersectable;
pub mod render;
pub mod tracer;
pub mod utils;
pub mod math;
pub mod geo;
 */


extern crate rand;

use std::fs::File;
use std::io::Write;
use std::f64::consts::PI;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::thread::{spawn, JoinHandle};
use std::sync::mpsc::{Sender, channel};
use std::convert::From;

use self::rand::Rng;
use self::rand::rngs::ThreadRng;


/* TYPE DEFINITIONS */

type Pixel      = (u64, u64);
type RGB        = (u8, u8, u8);
type RGBA       = (u8, u8, u8, u8);
type PairF64    = (f64, f64);
type TripleF64  = (f64, f64, f64);
pub type Sector = (u64, u64, u64, u64);
pub type Thread = JoinHandle<()>;


/* END TYPE DEFINITIONS */


/* CONSTANT DEFINITIONS */
const LOG2 : f64 = 0.6931471805599453;
/* END CONSTANT DEFINTIONS */


/* TRAIT DEFINITIONS */

// TODO: change t_min and t_max to use PairF64 tuple instead
pub trait RTObject {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Intersect;
}

// rendering trait
pub trait PPMRender {
    fn to_ppm(&self, c: &Camera, set: &Settings) -> Result<u8, String>;
}



/* END TRAIT DEFINITIONS */



/* ENUM DEFINITIONS */
pub enum Intersect {
    None,
    Hit(f64, V3, V3, Material),
}

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambert(V3),
    Metal(V3, f64),
    Glass(f64),
}

pub enum Incident {
    None,
    Refracted(V3),
    Scattered(V3, Ray),
}



// Enumeration used for multithreaded rendering
pub enum Msg {
    Draw(Pixel, RGB),
    End,
}


// This struct will be passed onto rendering threads.
// Each thread will have access to it's own RNG generator
// as to avoid having to re-initialize RNG instances every time
// we want to generate numbers. Which means we want to pass a reference
// to the ThreadData each time we do some kind of raytracing function
// to access RNG generation methods.
//
// The reference to a ThreadData should always be mutable, because
// the RNG generator is by itself required to be mutable.
pub struct ThreadData {
    pub section: Sector,
    pub rng:     ThreadRng, 
}

/* END ENUM DEFINITIONS */



/* STRUCT DEFINITIONS */
#[derive(Clone, Copy, Debug)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// TODO: add time at which ray was fired (raytracing book 2)
pub struct Ray {
    pub pos:   V3,
    pub dir:   V3,
    //pub time: f64,
}

pub struct Sphere {
    pub center:         V3,
    pub radius:        f64,
    pub material: Material,
}


// TODO: add BVH tree (raytracing book 2)
pub struct Scene {
    pub objects: Vec<Box<RTObject>>,
}


// TODO: add multithreading (look at sleibrock/rustybrot)
pub struct Settings {
    pub width:         u64,
    pub height:        u64,
    pub depth_limit:   u64,
    pub aa_samples:    u64,
    pub fname:      String,
    pub threads:       u64,
}


pub struct Camera {
    pub pos:         V3,
    pub target:      V3,
    pub view_up:     V3,
    pub vert_fov:   f64,
    pub aspect:     f64,
    pub aperture:   f64,
    pub focus_dist: f64,
}


pub struct PPMBuffer {
    pub width:    usize,
    pub height:   usize,
    pub size:     usize,
    pub buffer: Vec<u8>,
}


/* END STRUCT DEFINITIONS */



/* IMPLEMENTATIONS */
impl Settings {
    pub fn new(f: &str) -> Settings {
        Settings{
            width:       640,
            height:      480,
            depth_limit: 50,
            aa_samples:  1,
            fname:       String::from(f),
            threads:     1,
        }
    }

    pub fn width(mut self, w: u64) -> Settings {
        self.width = w; return self;
    }

    pub fn height(mut self, h: u64) -> Settings {
        self.height = h; return self;
    }

    pub fn aa_samples(mut self, aa: u64) -> Settings {
        self.aa_samples = aa; return self;
    }

    pub fn depth_limit(mut self, dl: u64) -> Settings {
        if dl > 100 {
            println!("[WARN] recursion limit may exceed memory");
        }
        self.depth_limit = dl; return self;
    }

    pub fn threads(mut self, t: u64) -> Settings {
        self.threads = t; return self;
    }
}


impl Camera {
    pub fn new(pos: V3) -> Camera {
        Camera {
            pos:       pos,
            target:    V3::i(), // (1, 0, 0)
            view_up:   V3::j(), // (0, 1, 0)
            vert_fov:  90.0,
            aspect:    1.0,
            aperture:  1.0,
            focus_dist: 1.0,
        }
    }

    pub fn pos(mut self, p: V3) -> Camera {
        self.pos = p; self
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
        let w = (self.pos - self.target).normal();
        let u = self.view_up.cross(&w).normal();
        let v = w.cross(&u);
        let bleft = self.pos
            - (half_width * u * self.focus_dist)
            - (half_height * v * self.focus_dist)
            - (w * self.focus_dist);
        let horiz = 2.0 * half_width * u * self.focus_dist;
        let verti = 2.0 * half_height * v * self.focus_dist;
        let rd = lens_radius * random_in_unit_disk();
        let offset = u * rd.x + v * rd.y;
        Ray{
            pos: self.pos + offset,
            dir: bleft+(horiz*s)+(verti*t)-self.pos-offset,
        }
    }
}





/// Vector class of 3 dimensions
/// Can also be used to store colors (x=r, y=g, z=b)
/// Arithmetic supported: Addition, Subtraction, Multiplication, Division
/// Supports: dot product, cross product, negation
/// Included: core unit vectors (i/j/k), ones and zeroes
impl V3 {
    pub fn new(nx: f64, ny: f64, nz: f64) -> V3 {
        V3 { x: nx, y: ny, z: nz }
    }

    // core unit vectors
    pub fn zeroes() -> V3 { (0., 0., 0.).into() }
    pub fn ones()   -> V3 { (1., 1., 1.).into() }
    pub fn i()      -> V3 { (1., 0., 0.).into() }
    pub fn j()      -> V3 { (0., 1., 0.).into() }
    pub fn k()      -> V3 { (0., 0., 1.).into() }

    
    // multiply the current vector by another vector
    pub fn product(&self, v: &V3) -> V3 {
        V3 { x: self.x*v.x, y: self.y*v.y, z: self.z*v.z }
    }

    
    // multiply the current vector by a scalar
    pub fn scale(&self, s: f64) -> V3 {
        (*self) * s
    }


    // divide the current vector by a divisor
    pub fn div(&self, d: f64) -> V3 {
        (*self) / d
    }


    // length functions
    pub fn length2(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }


    // calculate the normalized component of a vector
    pub fn normal(&self) -> V3 {
        let l = self.length();
        if l == 0.0 {
            return V3::zeroes();
        }
        return (*self) / l;
    }


    // this calculates the dot product between *self and &V3
    // It is better to use Vec*Vec for dot products to avoid ref passing
    pub fn dot(&self, o: &V3) -> f64 {
        self.x*o.x + self.y*o.y + self.z*o.z
    }


    // calculate the cross product between two vectors
    pub fn cross(&self, o: &V3) -> V3 {
        V3 {
            x: self.y*o.z - self.z*o.y,
            y: -(self.x*o.z - self.z*o.x),
            z: self.x*o.y - self.y*o.x,
        }
    }


    // Reflect a vector against a normal
    // v - 2*dot(v,n)*n
    pub fn reflect(&self, normal: &V3) -> V3 {
        self.copy() - normal.copy()*(2.0*self.dot(normal))
    }

    // sqrt all elements of the vector (used for gamma correction)
    pub fn sqrt(&self) -> V3 {
        V3 { x: self.x.sqrt(), y: self.y.sqrt(), z: self.z.sqrt() }
    }


    // used to full-copy a vector
    pub fn copy(&self) -> V3 {
        V3 { x: self.x, y: self.y, z: self.z }
    }


    // debug
    pub fn print(&self) {
        println!("vec: {} {} {}", self.x, self.y, self.z);
    }
}


// trait impls for math overriding
impl Add for V3 {
    type Output = V3;
    fn add(self, o: V3) -> V3 {
        V3 { x: self.x+o.x, y: self.y+o.y, z: self.z+o.z }
    }
}


impl Sub for V3 {
    type Output = V3;
    fn sub(self, o: V3) -> V3 {
        V3 { x: self.x-o.x, y: self.y-o.y, z: self.z-o.z }
    }
}


// a Vector * Vector should be considered a dot product
impl Mul for V3 {
    type Output = f64;
    fn mul(self, o: V3) -> f64 {
        self.x*o.x + self.y*o.y + self.z*o.z
    }
}


impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        V3 { x: -self.x, y: -self.y, z: -self.z }
    }
}


impl Mul<f64> for V3 {
    type Output = V3;
    fn mul(self, scalar: f64) -> V3 {
        V3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}


impl Mul<V3> for f64 {
    type Output = V3;
    fn mul(self, vector: V3) -> V3 {
        V3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}


impl Div<f64> for V3 {
    type Output = V3;
    fn div(self, divisor: f64) -> V3 {
        if divisor == 0.0 {
            panic!("Division by zero!");
        }
        V3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
        }
    }
}

impl From<V3> for TripleF64 {
    fn from(xyz: V3) -> Self {
        (xyz.x, xyz.y, xyz.z)
    }
}

impl From<TripleF64> for V3 {
    fn from((x, y, z): TripleF64) -> Self {
        V3::new(x, y, z)
    }
}


impl Ray {
    pub fn new(npos: TripleF64, ndir: TripleF64) -> Ray {
        Ray { pos: V3::from(npos), dir: V3::from(ndir), }
    }

    // Origin + (Dir*t)
    pub fn point_at(&self, t: f64) -> V3 {
        self.pos + self.dir * t
    }
}




impl Sphere {
    pub fn new(xyz: TripleF64, r: f64, m: Material) -> Sphere {
        Sphere{ center: V3::from(xyz), radius: r, material: m }
    }
}


impl RTObject for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Intersect {
        let oc   = ray.pos - self.center;
        let a    = ray.dir * ray.dir;
        let b    = oc * ray.dir;
        let c    = oc*oc  - self.radius*self.radius;
        let disc = b*b - a*c;

        if disc > 0.0 {
            let disc_sqrt = disc.sqrt();
            let t1 = (-b - disc_sqrt)/a;
            if t1 < t_max && t1 > t_min {
                return Intersect::Hit(
                    t1,
                    ray.point_at(t1),
                    (ray.point_at(t1) - self.center) / self.radius,
                    self.material,
                );
            }
            let t2 = (-b + disc_sqrt)/a;
            if t2 < t_max && t2 > t_min {
                return Intersect::Hit(
                    t2,
                    ray.point_at(t2),
                    (ray.point_at(t2) - self.center) / self.radius,
                    self.material,
                );
            }
        }
        return Intersect::None;
    }
}



// TODO: work on BVH
impl Scene {
    pub fn new() -> Scene {
        let v : Vec<Box<RTObject>> = Vec::new();
        Scene { objects: v }
    }

    pub fn add_object(&mut self, obj: Box<RTObject>) {
        self.objects.push(obj);
    }
}

// TODO: BVH
impl RTObject for Scene {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Intersect {
        let mut closest = t_max;
        let mut intersected = Intersect::None;

        for obj in &self.objects {

            match obj.intersect(r, t_min, closest) {
                Intersect::None => {}
                Intersect::Hit(t, pos, nrm, mat) => {
                    if t < closest {
                        closest = t;
                        intersected = Intersect::Hit(t, pos, nrm, mat);
                    }
                }
            }
        }
        return intersected;
    }
}


// implement a PPM renderer for the World object
// TODO: switch to PPMBuffer rendering method with threading/mpsc
impl PPMRender for Scene {
    fn to_ppm(&self, c: &Camera, set: &Settings) -> Result<u8, String> { 
        let mut f = match File::create(set.fname.as_str()) {
            Ok(file) => file,
            _ => panic!("Failed to open the file"),
        };

        match f.write(format!("P3\n{} {}\n255\n", set.width, set.height).as_bytes()) {
            Ok(_) => {},
            _ => panic!("Failed to write header"),
        }

        // set up thread rng
        let mut rng = rand::thread_rng();
        let mut col = V3::zeroes();

        for j in (0..set.height).rev() {
            for i in 0..set.width {
                col = V3::zeroes();

                // if AA samples is above zero, do random ray sampling
                if set.aa_samples != 0 {
                    for sc in 0..set.aa_samples {
                        let u = (i as f64 + rng.gen::<f64>()) / set.width as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / set.height as f64;
                        let r = c.get_ray(u, v);
                        col = col + trace(&r, self, set.depth_limit);
                    }
                } else {
                    // else just fire rays with no randomization
                    let u = i as f64 / set.width as f64;
                    let v = i as f64 / set.height as f64;
                    let r = c.get_ray(u, v);
                    col = col + trace(&r, self, set.depth_limit);
                }

                col = col / set.aa_samples as f64;
                col = col.sqrt(); 

                let ir = (255.99 * col.x) as u32;
                let ig = (255.99 * col.y) as u32;
                let ib = (255.99 * col.z) as u32;
                match f.write(format!("{} {} {}\n", ir, ig, ib).as_bytes()) {
                    Ok(_) => {},
                    _ => panic!("Failed to write bytes"),
                }
            }
        }

        Ok(0)
    }
}


impl Material {
    pub fn scatter(&self, r: &Ray, hit: Intersect) -> Incident {
        match *self {
            Material::Lambert(lv)     => calc_lambert(lv, r, hit),
            Material::Metal(mv, fuzz) => calc_metal(mv, fuzz, r, hit),
            Material::Glass(gv)       => calc_glass(gv, r, hit),
        }
    }  
}


/* END IMPLEMENTATIONS */



/* FUNCTIONS */

// MATH STUFF

pub fn sin(x: f64) -> f64 { x.sin() }
pub fn cos(x: f64) -> f64 { x.cos() }
pub fn tan(x: f64) -> f64 { x.tan() }
pub fn to_deg(x: f64) -> f64 { 0. }
pub fn to_rad(x: f64) -> f64 { 0. }



pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen::<f64>();
}

pub fn rand_pair() -> PairF64 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f64>(), rng.gen::<f64>())
}

pub fn rand_triple() -> TripleF64 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}

pub fn rand_vec() -> V3 {
    let mut rng = rand::thread_rng();
    return V3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
}

// other vector-related functions
// Get a random point inside a Unit Sphere (radius=1)
pub fn random_in_unit_sphere() -> V3 {
    let mut p = 2.0 * rand_vec() - V3::ones();
    while p.length2() >= 1.0 {
        p = 2.0 * rand_vec() - V3::ones();
    }
    return p;
}

// get a random point inside a Unit Disk (radius=1)
pub fn random_in_unit_disk() -> V3 {
    let mut p = 2.0 * rand_vec() - V3::ones();
    while p*p >= 1.0 {
        p = 2.0 * rand_vec() - V3::ones();
    }
    return p;
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



// define interactions here
pub fn calc_lambert(albedo: V3, r: &Ray, hit: Intersect) -> Incident {
    match hit {
        Intersect::Hit(t, p, nrm, _) => {
            let target = p + nrm + random_in_unit_sphere();
            Incident::Scattered(
                albedo,
                Ray{ pos: p, dir: target - p }
            )
        }
        _ => Incident::None,
    }
}

pub fn calc_metal(albedo: V3, fuz: f64, r: &Ray, hit: Intersect) -> Incident {
    match hit {
        Intersect::Hit(t, p, nrm, _) => {
            let refl = r.dir.reflect(&nrm);
            if refl*nrm > 0.0 {
                Incident::Scattered(
                    albedo,
                    Ray{
                        pos: p,
                        dir: refl + (fuz*random_in_unit_sphere())
                    }
                )
            } else {
                Incident::None
            }
        }
        _ => Incident::None,
    }
}

pub fn calc_glass(refrac: f64, r: &Ray, hit: Intersect) -> Incident {
    
    match hit {
        Intersect::Hit(t, p, nrm, _) => {
            let reflected = r.dir.reflect(&nrm);
            let atten = V3::ones();
            
            let mut outward_normal = V3::zeroes();
            let mut ni_over_nt = 0.;
            let mut refl_prob = 0.;
            let mut cosine = 0.;
            let mut refracted = V3::zeroes();
            let mut scray = Ray::new((0., 0., 0.), (0., 0., 0.));
            
            if r.dir*nrm > 0.0 {
                outward_normal = -nrm;
                ni_over_nt = refrac;
                cosine = (refrac*r.dir*nrm)/r.dir.length();
            } else {
                outward_normal = nrm;
                ni_over_nt = 1.0 / refrac;
                cosine = -(refrac*r.dir*nrm)/r.dir.length();
            }

            match refract(r.dir, outward_normal, ni_over_nt) {
                Incident::Refracted(rr) => {
                    refl_prob = schlick(cosine, refrac);
                    refracted = rr;
                }
                _ => {
                    scray = Ray{ pos: p, dir: reflected };
                    refl_prob = 1.0;
                }
            }

            if random() < refl_prob {
                scray = Ray{ pos: p, dir: reflected };
            } else {
                scray = Ray{ pos: p, dir: refracted };
            }
            
            Incident::Scattered(atten, scray)
        },
        
        _ => Incident::None,
    }
}




// short hand initializers for all boxed objects/assets
pub fn new_sphere(xyz: TripleF64, r: f64, m: Material) -> Box<Sphere> {
    Box::new(Sphere::new(xyz, r, m))
}


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


// count how many threads we will need with N subdivisions of the screen space
fn thread_count(div: u64) -> u64 {
    (4 as i64).pow((div as u32)-1) as u64
}





pub fn trace(r: &Ray, w: &Scene, limit: u64) -> V3 {
    let intersect = w.intersect(r, 0.001, 25000.0);
    match intersect {
        // object was detected, calculate the surface normal color
        Intersect::Hit(t, p, nrm, mat) => {
            if limit != 0 {
                match mat.scatter(r, intersect) {
                    Incident::Scattered(atten, scattered) => {
                        return trace(&scattered, w, limit-1).product(&atten);
                    },
                    _ => {
                        return V3::zeroes();
                    },
                }
            }
            return V3::zeroes();
        }
        
        // nothing hit, use a background color
        _ => {
            let ud = r.dir.normal();
            let t = 0.5*(ud.y + 1.0);
            return (1.0-t)*V3::ones() + t*V3::new(0.5, 0.7, 1.0);
        }
    }

}


/* END FUNCTIONS */



mod test {

}


// end lib.rs
