// v3.rs
// vector class

use std::ops::{Add, Sub, Mul, Neg, Div};
extern crate rand;
use self::rand::Rng;


#[derive(Clone, Copy, Debug)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    pub fn zeroes() -> V3 { V3 { x: 0.0, y: 0.0, z: 0.0 } }
    pub fn ones()   -> V3 { V3 { x: 1.0, y: 1.0, z: 1.0 } }
    pub fn i()      -> V3 { V3 { x: 1.0, y: 0.0, z: 0.0 } }
    pub fn j()      -> V3 { V3 { x: 0.0, y: 1.0, z: 0.0 } }
    pub fn k()      -> V3 { V3 { x: 0.0, y: 0.0, z: 1.0 } }

    // multiply the current vector by another vector
    pub fn product(&self, v: &V3) -> V3 {
        V3 { x: self.x*v.x, y: self.y*v.y, z: self.z*v.z }
    }

    
    // multiply the current vector by a scalar
    pub fn scale(&self, s: f64) -> V3 {
        V3 { x: self.x*s, y: self.y*s, z: self.z*s }
    }


    // divide the current vector by a divisor
    pub fn div(&self, d: f64) -> V3 {
        if d == 0.0 {
            panic!("Division by zero!");
        }
        V3 { x: self.x/d, y: self.y/d, z: self.z/d }
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
        V3 { x: self.x/l, y: self.y/l, z: self.z/l }
    }


    // this calculates the dot product between *self and &V3
    // It is better to use Vec*Vec for dot products to avoid ref passing
    pub fn dot(&self, o: &V3) -> f64 {
        self.x*o.x + self.y*o.y + self.z*o.z
    }


    // calculate the cross product between two vectors
    pub fn cross(&self, o: &V3) -> V3 {
        V3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }


    // Reflect a vector against a normal
    // v - 2*dot(v,n)*n
    pub fn reflect(&self, normal: &V3) -> V3 {
        self.copy() - normal.copy()*(2.0*self.dot(normal))
    }

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

// other vector-related functions
pub fn random_in_unit_sphere() -> V3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0*V3::new(
        rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()
    ) - V3::ones();
    while p.length2() >= 1.0 {
        p = 2.0*V3::new(
            rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()
        ) - V3::ones();
    }
    return p;
}

// end v3.rs
