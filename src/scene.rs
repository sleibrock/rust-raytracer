
use vector::*;
use geometry::*;
use shading::*;
use camera::*;
use ppm::*;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<Intersectable>>,
    pub lights: Vec<Light>,
}


impl Scene {

    // Create a new scene with a given camera
    pub fn new(c: Camera) -> Scene {
        return Scene{
            camera: c,
            objects: vec![],
            lights: vec![],
        }
    }


    // add an object to the scene only if it is intersectable
    // return an whatever 
    pub fn add_object(&mut self, obj: Box<Intersectable>) -> usize {
        self.objects.push(obj);
        return self.objects.len();
    }


    // inject a new light into the scene
    pub fn add_light(&mut self, light: Light) -> usize {
        self.lights.push(light);
        return self.objects.len();
    }



    // render all objects in the scene
    pub fn render(&self, w: u64, h: u64, fname: &str) {
        let mut output = PPM::new(w, h);

        for x in 0..w {
            for y in 0..h {
                let r = Ray{
                    pos: V3::new(x as f64, y as f64, 0.0),
                    dir: self.camera.dir,
                };

                let mut hit = false;

                for obj in &self.objects {
                    match obj.ray_intersect(&r) {
                        Some(_) => { hit = true; },
                        _ => {},
                    }
                }
                    
                if hit {
                    output.set_pixel(x, y, 255, 255, 255);
                }
            }
        }

        // write to file
        match output.to_file(fname) {
            Ok(_) => { println!("File rendered!"); },
            _     => { panic!("HELP ME!"); },
        }
    }
}
