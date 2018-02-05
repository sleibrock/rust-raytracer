extern crate raytracer;


use raytracer::vector::*;
use raytracer::shading::*;
use raytracer::geometry::*;
use raytracer::ppm::*;


pub fn ray_hits_sphere(r: &Ray, s: &Sphere) -> bool {
    let a = r.dir * r.dir;
    let dist = r.pos - s.pos;
    let b = 2.0 * (r.dir * dist);
    let c = (dist*dist) - (s.radius * s.radius);

    let disc = (b*b) - (4.0 * a * c);

    // ray never hits, return a negative value
    if disc < 0.0 {
        return false;
    }
    return true;

}

fn z_depth_rendering() {

    let width  : u32 = 1280;
    let height : u32 = 720;


    // add colors (these won't matter at all in this)
    let col = Color{r: 1.0, g: 1.0, b: 1.0};
    let mat = Material{diffuse: col, reflection: 1.0};

    let s = Sphere{
        pos:V3::new(20.0, 20.0, 20.0),
        radius: 10.0,
        mat: mat,
    };

    let direc = V3::new(0.0, 0.0, 1.0);

    let mut output = PPM::new(width, height);

    

    // begin looping and shooting rays really boring
    for x in 0..width {
        for y in 0..height {
            let r = Ray{
                pos: V3::new(x as f64, y as f64, 0.0),
                dir: direc,
            };

            let hit = ray_hits_sphere(&r, &s);

            if hit {
                output.set_pixel(x, y, 255, 255, 255);
            }

        }
    }



    // output the file
    match output.to_file("output.ppm") {
        Ok(_) => { println!("File rendered"); },
        _     => { panic!("Um"); }
    }

}


fn main() {
    z_depth_rendering();
}
