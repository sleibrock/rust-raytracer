// vector code

use std::ops::{Add, Sub, Mul, Neg};


#[derive(Debug)]
pub struct V3
{
    pub   x: f64,
    pub   y: f64,
    pub   z: f64,
    pub mag: f64,
}


// use to compute the magnitude of a given vector's coordinates
fn magnitude(x: f64, y: f64, z: f64) -> f64 {
    return (x*x) + (y*y) + (z*z);
}


impl V3 {

    pub fn new(x: f64, y: f64, z: f64) -> V3 {
        return V3{
            x: x, y: y, z: z,
            mag: magnitude(x, y, z),
        }
    }

    pub fn zero() -> V3 {
        return V3::new(0.0, 0.0, 0.0);
    }


    // Multiply the current vector by a scalar value and
    // return the new vector
    pub fn scale(&self, scalar: f64) -> V3 {
        return V3::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }


    // return a vector's normal (direction)
    pub fn normal(&self) -> V3 {
        if self.mag == 0.0 {
            panic!("Zero division error! (V3::normal)");
        }
        
        return V3::new(
            self.x / self.mag,
            self.y / self.mag,
            self.z / self.mag
        )
    }


    // return the dot product of two vectors (returns a float)
    // A * B = (a.x*b.x) + (a.y*b.y) + (a.z*b.z)
    pub fn dot(&self, other: V3) -> f64 {
        return (self.x*other.x) + (self.y*other.y) + (self.z*other.z);
    }


    // return the cross product between two vectors
    pub fn cross(&self, other: V3) -> V3 {
        return V3::new(0.0, 0.0, 0.0);
    }
}


impl Add for V3 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        return V3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        );
    }
}

impl Sub for V3 {
    type Output = V3;
    fn sub(self, other: V3) -> V3 {
        return V3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        );
    }
}


impl Mul for V3 {
    type Output = f64;
    fn mul(self, other: V3) -> f64 {
        return self.dot(other);
    }
}

impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        return V3::new(
            -self.x, -self.y, -self.z
        );
    }
}


impl PartialEq for V3 {
    fn eq(&self, other: &V3) -> bool {
        return (self.x==other.x) && (self.y==other.y) && (self.z==other.z);
    }
}


impl Copy for V3 {}
impl Clone for V3 {
    fn clone(&self) -> V3 {
        return *self;
    }
}



// run unit tests
#[cfg(test)]
mod tests {

    use vector::*;


    #[test]
    fn test_equality() {
        let a = V3::new(1.0, 1.0, 1.0);
        let b = V3::zero();
        
        assert_eq!(a, a);
        assert_eq!(a+b, a);
    }

    #[test]
    fn test_addition() {
        let a = V3::new(1.0, 0.0, 0.0);
        let b = V3::new(0.0, 1.0, 0.0);
        let c = V3::new(1.0, 1.0, 0.0);

        assert_eq!(a+b, c);

    }


    #[test]
    fn test_subtraction() {

    }

    
    #[test]
    fn test_multiplication_scalar() {
        let a = V3::new(1.0, 1.0, 1.0);
        let b = V3::new(2.0, 2.0, 2.0);

        assert_eq!(a.scale(2.0), b);
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

    #[test]
    fn test_cross() {
        
    }
    
}
