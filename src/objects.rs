
use std::mem::swap;
use std::ops::{Add, Sub, Mul, Neg};

// struct definitions
pub struct V3 {
    pub   x: f32,
    pub   y: f32,
    pub   z: f32,
    pub mag: f32,
}


pub struct Sphere {
    pub    pos:  V3,
    pub radius: f32,
}

pub struct Ray {
    pub    pos: V3,
    pub    dir: V3,
}


pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}


pub struct Light {
    pub       pos:    V3,
    pub intensity: Color,
}


pub struct Material {
    pub    diffuse: Color,
    pub reflection:   f32,
}


// use to compute the magnitude of a given vector's coordinates
fn magnitude(x: f32, y: f32, z: f32) -> f32 {
    return (x*x) + (y*y) + (z*z);
}


// vector3 definition
impl V3 {

    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        return V3{
            x: x, y: y, z: z,
            mag: magnitude(x, y, z)
        };
    }


    pub fn add(&self, other: V3) -> V3 {
        return V3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            mag: magnitude(self.x+other.x, self.y+other.y, self.z+other.z)
        };
    }


    pub fn sub(&self, other: V3) -> V3 {
        return V3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            mag: magnitude(self.x-other.x, self.y-other.y, self.z-other.z)
        };
    }


    pub fn mul(&self, scalar: f32) -> V3 {
        return V3{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            mag: magnitude(self.x*scalar, self.y*scalar, self.z*scalar)
        };
    }


    pub fn div(&self, scalar: f32) -> V3 {
        if scalar == 0.0 {
            panic!("Zero Division error");
        }

        return V3{
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            mag: magnitude(self.x/scalar, self.y/scalar, self.z/scalar)
        };
    }


    pub fn negate(&self) -> V3 {
        return V3{
            x: self.x * (-1.0),
            y: self.y * (-1.0),
            z: self.z * (-1.0),
            mag: magnitude(self.x*(-1.0), self.y*(-1.0), self.z*(-1.0))
        };
    }


    pub fn dot(&self, other: V3) -> f32 {
        return (self.x*other.x) + (self.y*other.y) + (self.z*other.z);
    }


    pub fn normal(&self) -> V3 {
        return self.div(self.mag);
    }
}
//end vector3


// implement minor traits for Vectors
impl PartialEq for V3 {
    fn eq(&self, other: &V3) -> bool {
        return (self.x == other.x)
            && (self.y == other.y)
            && (self.z == other.z);
    }
}


impl Copy for V3 {}
impl Clone for V3 {
    fn clone(&self) -> V3 {
        return *self;
    }
}



impl Ray {

    pub fn hits_sphere(&self, s: &Sphere) -> bool {
        let a = self.dir.dot(self.dir);
        
        let dist = self.pos.sub(s.pos);
        
        let b = 2.0 * (self.dir.dot(dist));
        let c = dist.dot(dist) - (s.radius*s.radius);
        
        let disc = (b*b) - (4.0*a*c);
        
        if disc < 0.0 {
            return false;
        } else {
            return true;
        }
    }
}



// run unit tests
#[cfg(test)]
mod tests {

    #[test]
    fn test_addition() {

    }


    #[test]
    fn test_subtraction() {

    }

    
    #[test]
    fn test_multiplication_scalar() {

    }


    #[test]
    fn test_dotproduct() {

    }


    #[test]
    fn test_division() {

    }


    #[test]
    fn test_normals() {

    }
    
}



// end
