use vector::*;

/*
Shading tools and other utilities for coloring

*/


// A color tuple struct representing the intensity
// of a single Pixel value
// Color indices should always be 0..1
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}


// A light represents a singular light source
// in the scene. Intensity describes the strength of
// the color from the light
pub struct Light {
    pub       pos:    V3,
    pub intensity: Color, 
}


// A Material describes the color and the
// reflectiveness of a geometric object for shading.
// 0 represents nonreflectiveness while 1 reps full
// reflection
pub struct Material {
    pub    diffuse: Color,
    pub reflection:   f64,
}
