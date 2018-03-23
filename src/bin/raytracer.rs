// raytracer.rs
// will be one file until broken up (eventually)

extern crate raytracer;

use raytracer::v3::*;
use raytracer::element::*;
use raytracer::camera::*;
use raytracer::world::*;
use raytracer::material::*;
use raytracer::render::*;
use raytracer::utils::*;


fn main() {
    // create a new scene
    let mut w = World::new();

    let width  = 1920;
    let height = 1080;
    let rat    = width as f64 / height as f64;

    let cam_pos = V3::new(3.0, 3.0, 2.0);
    let look_at = V3::new(0.0, 0.0, -1.0);
    
    // create a new camera with some information
    let camera = Camera::new(cam_pos) // init position
        .target(look_at)              // the target
        .fov(20.0)                    // (vertical) field of view
        .aspect_ratio(rat)            // aspect ratio of the image
        .aperture(0.1)                // aperture of the camera
        .focus((cam_pos - look_at).length()); // focus distance

    // create render settings for the output image
    let settings = Settings::new("test.ppm")
        .width(width)       // the width of the image
        .height(height)     // height of the img
        .aa_samples(100)    // how many rays per pixel for antialiasing
        .depth_limit(100);  // what the maximum recursion limit should be
    
    // add a bunch of random spheres
    // floor sphere
    w.push(new_sphere(0.0, -1000.0, 0.0, 1000.0, lambert(0.5, 0.5, 0.5)));
    
    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat = random();
            let c = V3::new((a as f64)*0.9*random(),
                            0.2,
                            (b as f64)*0.9*random());

            if (c-V3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    w.push(new_sphere(
                        c.x, c.y, c.z, 0.2,
                        lambert(random(), random(), random()),
                    ));
                } else if choose_mat < 0.95 {
                    w.push(new_sphere(
                        c.x, c.y, c.z, 0.2,
                        metal(0.5*(1.0+random()), 0.5*(1.0+random()), 0.5*(1.0+random()), 0.5*random()) 
                    ));
                } else {
                    w.push(new_sphere(c.x, c.y, c.z, 0.2, glass(random()*2.0)));
                }
            } 
        }
    }

    w.push(new_sphere(0.0, 1.0, 0.0, 1.0, glass(1.5)));
    w.push(new_sphere(-4.0, 1.0, 0.0, 1.0, lambert(0.4, 0.2, 0.1)));
    w.push(new_sphere(4.0, 1.0, 0.0, 1.0, metal(0.7, 0.6, 0.5, 0.0)));
    

    // render it to a PPM file
    match w.to_ppm(&camera, &settings) {
        Ok(_) => println!("File rendered"),
        _     => panic!("Failed to render image"),
    };
}


// end
