// src/lib.rs




// external libraries
pub mod aliases;
pub mod math;
pub mod ppm;

extern crate rand;

use std::fs::File;
use std::io::Write;
use std::thread::{spawn};
use std::sync::mpsc::{Sender, channel};

use self::rand::Rng;
use self::rand::rngs::ThreadRng;


pub use math::*;
pub use aliases::*;
pub use ppm::*;



/* TRAIT DEFINITIONS */

// TODO: change t_min and t_max to use PairF64 tuple instead
pub trait RTObject {
    fn intersect(&self, &Ray, f64, f64) -> Intersect;
    //fn intersect(&self, &Ray, PairF64) -> Intersect;
}

// rendering trait
pub trait PPMRender {
    fn to_ppm(&self, &Camera, &Settings) -> Result<u8, String>;
    //fn to_ppm(&self, &Camera, &Settings) -> Result<(), String>;
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




/* END STRUCT DEFINITIONS */



/* IMPLEMENTATIONS */
impl Settings {
    pub fn new(f: &str) -> Settings {
        Settings{
            width:       DEFAULT_WIDTH,
            height:      DEFAULT_HEIGHT,
            depth_limit: DEFAULT_DEPTH_LIMIT,
            aa_samples:  DEFAULT_AA_SAMPLES,
            fname:       String::from(f),
            threads:     DEFAULT_THREADS,
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
            vert_fov:  DEFAULT_FOV,
            aspect:    DEFAULT_ASPECT_RAT,
            aperture:  DEFAULT_APERTURE,
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


// start doing the actual raytracing stuff here

fn draw_thread(tx: &Sender<Msg>, td: ThreadData) -> Thread {
    let sender = Sender::clone(tx);
    let(x_min, y_min, x_max, y_max) = td.section;

    return spawn(move ||{
        for x in x_min .. x_max {
            for y in y_min .. y_max {
                let p = get_pixel((x,y));
            }
        }
        sender.send(Msg::End).unwrap();
    });
}

fn get_pixel(xy: Pixel) -> RGB {
    (0,0,0)
}






/* FUNCTIONS */


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
    (rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()).into()
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



// Define all material interactions below
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

pub fn metal(xyz: TripleF64, fuzz: f64) -> Material {
    Material::Metal(xyz.into(), match fuzz < 1.0 {
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



/// Subdivide a Sector (x1, y1, x2, y2) into a number of subdivided regions.
/// `div` is the total number of subdivisions made.
fn subdivide(div: u64, sect: Sector) -> Vec<Sector> {
    let (x1, y1, x2, y2) = sect;
    let mut buf : Vec<Sector> = Vec::new();
    let width  = x2 - x1;
    let height = y2 - y1;
    let srw    = width / div;
    let srh    = height / div;
    for x in 0 .. div {
        for y in 0 .. div {
            buf.push((x*srw, y*srh, (x+1)*srw, (y+1)*srh));
        }
    }
    return buf;
}


// create a new rendering thread by passing a Sender and a ThreadData struct




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
