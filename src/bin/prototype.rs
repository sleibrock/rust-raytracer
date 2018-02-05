extern crate raytracer;

use raytracer::vector::*;
use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shading::*;
use raytracer::geometry::*;


fn main() {

    // create a new camera at origin with a (1,0,0) direction
    let cam = Camera::new(V3::zero(), V3::new(0.0, 0.0, 1.0));

    // define a new scene
    let mut scene = Scene::new(cam);


    // define a sphere and add it to the scene
    let color = Color{r: 1.0, g: 1.0, b: 1.0};
    let mat = Material{diffuse: color, reflection: 1.0};

    let s = Sphere{
        pos: V3::new(20.0, 20.0, 20.0),
        radius: 10.0,
        mat: mat,
    };

    scene.add_object(Box::new(s));


    // render the scene into a PPM file
    let width  = 1280;
    let height =  720;

    scene.render(width, height, "file.ppm");
}
