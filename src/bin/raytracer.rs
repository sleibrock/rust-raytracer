// raytracer.rs
// Loads up necessary components and renders a fully raytraced scene

extern crate raytracer;

use raytracer::*;
use std::f64::consts::PI;

fn main() {
    // create a new scene
    let mut w = Scene::new();

    // define some output settings
    let width  = 640; 
    let height = 480;
    let rat    = width as f64 / height as f64;

    let cam_pos = V3::new(5.5, 3.0, 1.0);
    let look_at = V3::new(0.0, 2.0, 0.0);
    
    let camera = Camera::new(cam_pos) // init position
        .target(look_at)              // the target
        .fov(75.0)                    // (vertical) field of view
        .aspect_ratio(rat)            // aspect ratio of the image
        .aperture(0.1)                // aperture of the camera
        .focus((cam_pos - look_at).length()); // focus distance

    // create render settings for the output image
    let settings = Settings::new("test.ppm")
        .width(width)                 // width of the image
        .height(height)               // height of the image
        .aa_samples(25)               // ray count avg per pixel
        .depth_limit(50);             // maximum recursion limit for tracing 
    
    // add a bunch of random spheres
    // floor sphere
    w.add_object(new_sphere((0., -1000., 0.), 1000.0, lambert(0.1, 0.3, 0.1)));

    let radius = 4.0;
    let twopi = 2.0 * PI;

    let red   = [1., 0.1, 0.1, 1., 1., 0.1, 0.8, 0.2];
    let green = [0.1, 1., 0.1, 1., 0.1, 1., 0.8, 0.2];
    let blue  = [0.1, 0.1, 1., 0.1, 1.0, 1., 0.8, 0.2];
    for i in 0..8 {
        let current_angle = (i as f64 / 8.0) * twopi;
        let other_angle = (i as f64 / 8.0) * PI;
        let sx = current_angle.cos() * radius;
        let sz = current_angle.sin() * radius;

//        let sy = (((other_angle*0.5).sin()+1.0)*0.5); 
        let sy = (other_angle.sin())*2.0;
        println!("radians: {}, sy: {}", other_angle, sy);
        
        w.add_object(new_sphere((sx, 1.0+sy, sz), 1.0, match (i%2)==0 {
            true => lambert(red[i], green[i], blue[i]),
            _    => metal((red[i], green[i], blue[i]), 1.0),
        }));
    }

    w.add_object(new_sphere((0., 3., 0.), 2.0, glass(1.1)));

    // render it to a PPM file
    match w.to_ppm(&camera, &settings) {
        Ok(_) => println!("File rendered"),
        _     => panic!("Failed to render image"),
    };
}


// end
