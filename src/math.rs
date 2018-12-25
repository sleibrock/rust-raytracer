use std::ops::{Add, Sub, Mul, Div, Neg};
use std::convert::From;
pub use std::f64::consts::PI;

use aliases::*;

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
    pub fn zeroes() -> V3 { (0., 0., 0.).into() }
    pub fn ones()   -> V3 { (1., 1., 1.).into() }
    pub fn i()      -> V3 { (1., 0., 0.).into() }
    pub fn j()      -> V3 { (0., 1., 0.).into() }
    pub fn k()      -> V3 { (0., 0., 1.).into() }

    
    // multiply the current vector by another vector
    pub fn product(&self, v: &V3) -> V3 {
        (self.x*v.x, self.y*v.y, self.z*v.z).into()
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
        (self.y*o.z-self.z*o.y, -(self.x*o.z-self.z*o.x), self.x*o.y-self.y*o.x).into()
    }


    // Reflect a vector against a normal
    // v - 2*dot(v,n)*n
    pub fn reflect(&self, normal: &V3) -> V3 {
        self.copy() - normal.copy()*(2.0*self.dot(normal))
    }

    // sqrt all elements of the vector (used for gamma correction)
    pub fn sqrt(&self) -> V3 {
        (self.x.sqrt(), self.y.sqrt(), self.z.sqrt()).into()
    }


    // used to full-copy a vector
    pub fn copy(&self) -> V3 {
        (self.x, self.y, self.z).into()
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
        (self.x + o.x, self.y + o.y, self.z + o.z).into()
    }
}


impl Sub for V3 {
    type Output = V3;
    fn sub(self, o: V3) -> V3 {
        (self.x - o.x, self.y - o.y, self.z - o.z).into()
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
        (-self.x, -self.y, -self.z).into()
    }
}


impl Mul<f64> for V3 {
    type Output = V3;
    fn mul(self, s: f64) -> V3 {
        (self.x * s, self.y * s, self.z * s).into()
    }
}


impl Mul<V3> for f64 {
    type Output = V3;
    fn mul(self, vector: V3) -> V3 {
        (vector.x * self, vector.y * self, vector.z * self).into()
    }
}


impl Div<f64> for V3 {
    type Output = V3;
    fn div(self, divisor: f64) -> V3 {
        if divisor == 0.0 {
            panic!("Division by zero!");
        }
        (self.x / divisor, self.y / divisor, self.z / divisor).into()
    }
}



/// From<T> implementation for TripleF64 to convert TripleF64 into a V3.
impl From<TripleF64> for V3 {
    fn from((x, y, z): TripleF64) -> Self {
        V3::new(x, y, z)
    }
}

/// From<T> implementation for TripleF64. Used to convert V3 back into a tuple.
impl From<V3> for TripleF64 {
    fn from(xyz: V3) -> Self {
        (xyz.x, xyz.y, xyz.z)
    }
}



// anything else

// wrappers for trig because I don't like chaining these
pub fn sin(x: f64) -> f64 { x.sin() }
pub fn cos(x: f64) -> f64 { x.cos() }
pub fn tan(x: f64) -> f64 { x.tan() }
pub fn to_deg(x: f64) -> f64 { 0. }
pub fn to_rad(x: f64) -> f64 { 0. }



mod test {

}
