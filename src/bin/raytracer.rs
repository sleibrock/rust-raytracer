// raytracer.rs
// will be one file until broken up (eventually)

extern crate raytracer;

use raytracer::element::*;
use raytracer::camera::*;
use raytracer::world::*;
use raytracer::material::*;
use raytracer::render::*;


fn main() {
    // create a new scene
    let mut w = World::new();

    // create a new (stock) camera
    let camera = Camera::new();

    // create render settings for the output image
    let settings = Settings::new("test.ppm")
        .width(800)        // the width of the image
        .height(400)       // height of the img
        .aa_samples(100)   // how many rays per pixel for antialiasing
        .depth_limit(50);  // what the maximum recursion limit should be
    
    // inject spheres into the scene
    w.push(new_sphere( 0.0, 0.0, -1.0, 0.5, lambert(0.3, 0.5, 0.7)));
    w.push(new_sphere( 1.0, 0.0, -1.0, 0.5, lambert(0.5, 0.7, 0.1)));
    w.push(new_sphere(-1.0, 0.0, -1.0, 0.5, lambert(0.1, 0.1, 0.1)));
    
    // floor sphere
    w.push(new_sphere(0.0, -100.5, -1.0, 100.0, lambert(0.0, 1.0, 0.1)));

    // render it to a PPM file
    match w.to_ppm(&camera, &settings) {
        Ok(_) => println!("File rendered"),
        _     => panic!("Failed to render image"),
    };
}


// end
