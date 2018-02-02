extern crate raytracer;

use raytracer::objects::*;


// attempt to create a raytracer in a text-only format
fn text_renderer() {

    let s = Sphere{
        pos:V3::new(20.0, 20.0, 20.0),
        radius: 10.0
    };


    let direc = V3::new(0.0, 0.0, 1.0);


    for x in 0..40 {

        for y in 0..40 {
            let r = Ray{
                pos: V3::new(x as f32, y as f32, 0.0),
                dir: direc,
            }; 

            let hit = r.hits_sphere(&s);

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
