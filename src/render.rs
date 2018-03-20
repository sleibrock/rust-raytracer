// render.rs

extern crate rand;

use std::fs::File;
use std::io::Write;
use self::rand::Rng;

use v3::*;
use ray::*;
use element::*;
use camera::*;
use world::*;
use tracer::*;


pub struct Settings {
    pub width:         u64,
    pub height:        u64,
    pub depth_limit:   u64,
    pub aa_samples:    u64,
    pub fname:      String,
}


// use a method chaining style object initializer
// This way a stock Settings is always filled with default settings
// that can easily be modified by it's methods, as opposed to one hard-to-read
// initializer method
impl Settings {
    pub fn new(f: &str) -> Settings {
        Settings{
            width:       640,
            height:      480,
            depth_limit: 50,
            aa_samples:  1,
            fname:       String::from(f)
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
}




// rendering trait
pub trait PPMRender {
    fn to_ppm(&self, c: &Camera, set: &Settings) -> Result<u8, String>;
}


// implement a PPM renderer for the World object
impl PPMRender for World {
    fn to_ppm(&self, c: &Camera, set: &Settings) -> Result<u8, String> { 
        let mut f = match File::create(set.fname.as_str()) {
            Ok(file) => file,
            _ => panic!("Failed to open the file"),
        };

        match f.write(format!("P3\n{} {}\n255\n",
                              set.width, set.height).as_bytes()) {
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


// end render.rs
