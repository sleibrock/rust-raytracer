// utils.rs
// shortcut helper functions

extern crate rand;
use self::rand::Rng;

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

// end utils.rs
