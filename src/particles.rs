use lazy_static;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

lazy_static! {
    static ref ZERO: Vec3 = Vec3 { x: 0, y: 0, z: 0 };
}
impl Vec3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn zero() -> &'static Self {
        &ZERO
    }
    pub fn dist(&self, o: &Vec3) -> i64 {
        (self.x - o.x).abs() + (self.y - o.y).abs() + (self.z - o.z).abs()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Particle3 {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}
impl Particle3 {
    pub fn new(p: Vec3, v: Vec3, a: Vec3) -> Self {
        Particle3 { p, v, a }
    }
}
