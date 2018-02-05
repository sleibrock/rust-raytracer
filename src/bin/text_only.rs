extern crate raytracer;

use raytracer::vector::*;
use raytracer::shading::*;
use raytracer::geometry::*;


/*
A text-only Raytracer that tests rays from a wall
of rays to hit a single sphere in a scene
*/


pub fn ray_hits_sphere(r: &Ray, s: &Sphere) -> bool {
    let a = r.dir * r.dir;
    let dist = r.pos - s.pos;
    let b = 2.0 * (r.dir * dist);
    let c = (dist*dist) - (s.radius * s.radius);

    let disc = (b*b) - (4.0 * a * c);

    if disc < 0.0 {
        return false;
    } else {
        return true;
    }
}


// attempt to create a raytracer in a text-only format
fn text_renderer() {


    // add colors (these won't matter at all in this)
    let col = Color{r: 1.0, g: 1.0, b: 1.0};
    let mat = Material{diffuse: col, reflection: 1.0};

    let s = Sphere{
        pos:V3::new(20.0, 20.0, 20.0),
        radius: 10.0,
        mat: mat,
    };

    let direc = V3::new(0.0, 0.0, 1.0);


    for x in 0..40 {

        for y in 0..40 {
            let r = Ray{
                pos: V3::new(x as f64, y as f64, 0.0),
                dir: direc,
            }; 

            let hit = ray_hits_sphere(&r, &s);

            if hit {
                print!("++");
            } else {
                print!("--");
            }

        }

        print!("\n");
    }
}



// a scrappy main function only used to initially build a "raytracer"
fn main() {
    text_renderer();
}
