use std::path::Path;
use std::fs::File;
use std::io::{Result, Write};


pub struct PPM {
    height: u32,
    width: u32,
    data: Vec<u8>,
}


impl PPM {


    pub fn new(width: u32, height: u32) -> PPM {
        let size = 3 * height * width;
        let buffer = vec![0; size as usize]; // set entire vec to zero

        return PPM{
            height: height,
            width:   width,
            data:   buffer,
        };
    }


    // the size of the vector buffer allocated to this PPM file
    pub fn buffer_size(&self) -> u32 {
       return 3*self.height*self.width;
    }
    

    // the offset to use in the vector for allocating data
    pub fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width * 3) + (x * 3);
        if offset < self.buffer_size() {
            return Some(offset as usize);
        }
        return None;
    }


    // allocate a pixel's color in the data vector
    // pattern match by using the internal offset
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset]     = r;
                self.data[offset + 1] = g;
                self.data[offset + 2] = b;
                return true;
            },
            None => { return false; },
        }
    }


    pub fn to_file(&self, filename: &str) -> Result<()> {
        let path = Path::new(filename);
        let mut file = try!(File::create(&path));
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        try!(file.write(header.as_bytes()));
        try!(file.write(&self.data));
        Ok(())
    }
}
    

// end ppm
