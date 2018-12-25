use std::thread::JoinHandle;


pub type Pixel      = (u64, u64);
pub type RGB        = (u8, u8, u8);
//type RGBA       = (u8, u8, u8, u8);
pub type PairF64    = (f64, f64);
pub type TripleF64  = (f64, f64, f64);
pub type Sector = (u64, u64, u64, u64);
pub type Thread = JoinHandle<()>;

pub const DEFAULT_WIDTH       : u64 = 640;
pub const DEFAULT_HEIGHT      : u64 = 480;
pub const DEFAULT_FOV         : f64 = 90.;
pub const DEFAULT_APERTURE    : f64 = 1.;
pub const DEFAULT_ASPECT_RAT  : f64 = 1.;
pub const DEFAULT_AA_SAMPLES  : u64 = 1;
pub const DEFAULT_DEPTH_LIMIT : u64 = 50;
pub const DEFAULT_THREADS     : u64 = 1;

// end
