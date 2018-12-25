use std::fs::File;
use std::io::Write;

use aliases::*;

pub struct PPMBuffer {
    pub width:    u64,
    pub height:   u64,
    pub size:     usize,
    pub buffer: Vec<u8>,
}

impl PPMBuffer {
    pub fn new(width: u64, height: u64) -> PPMBuffer {
        let size = (width * height * 3) as usize;
        return PPMBuffer {
            width:  width,
            height: height,
            size:   size,
            buffer: vec![0; size],
        };
    }

    pub fn set_pix(&mut self, xy: Pixel, rgb: RGB) -> bool {
        let (x, y) = xy;
        let (r, g, b) = rgb;
        let offset = ((y * self.width * 3) + (x * 3)) as usize;
        if offset > self.size {
            return false;
        }
        self.buffer[offset] = r;
        self.buffer[offset + 1] = g;
        self.buffer[offset + 2] = b;
        return true;
    }

    pub fn to_file(&self, fname: String) -> Result<(), String> {
        let mut f = match File::create(fname.as_str()) {
            Ok(file) => file,
            Err(_) => { return Err("Failed to open file".into()); } 
        };

        match f.write(format!("P6\n{} {}\n255\n",
                              self.width, self.height).as_bytes()) {
            Err(_) => { return Err("Failed to write header".into()); }
            _ => {}
        }

        match f.write(&self.buffer) {
            Err(_) => { return Err("Failed to write buffer".into()); }
            _ => {}
        }

        return Ok(());
    }
}
