use vector::*;


pub struct Camera {
    pub pos: V3,
    pub dir: V3,

    
}

impl Camera {
    pub fn new(p: V3, d: V3) -> Camera {
        return Camera{
            pos: p,
            dir: d,
        };
    }


    pub fn set_pos(&mut self, np: V3) {
        self.pos = np;
    }

    pub fn set_dir(&mut self, nd: V3) {
        self.dir = nd;
    }

}
