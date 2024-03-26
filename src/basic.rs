use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::ops::{Index, IndexMut};
use std::default::Default;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { x: Default::default(), y: Default::default(), z: Default::default() }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn to_unit_len(&mut self) {
        let l = self.len();
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// scalar multiplication
impl Mul<f32> for Vec3{
    type Output = Vec3;

    #[inline]
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// element-wise multiplication
#[inline]
pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

// cross product
#[inline]
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pix {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pix {
    pub fn new() -> Pix {
        Pix {
            r: Default::default(),
            g: Default::default(),
            b: Default::default()
        }
    }
    pub fn from_vec3(v: Vec3) -> Pix {
        let mut pix = Pix::new();
        pix.set_float(v.x, v.y, v.z);
        return pix
    }
    // set the pixel values with u8
    #[inline]
    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
    
    // set the pixel values with f32, range [0.0, 1.0]
    #[inline]
    pub fn set_float(&mut self, r: f32, g: f32, b: f32) {
        self.r = (255.99 * r) as u8;
        self.g = (255.99 * g) as u8;
        self.b = (255.99 * b) as u8;
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}\n", self.r, self.g, self.b)
    }
}

impl Add for Pix {
    type Output = Pix;

    #[inline]
    fn add(self, other: Pix) -> Pix {
        Pix {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Pix {
    #[inline]
    fn add_assign(&mut self, other: Pix) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl Sub for Pix {
    type Output = Pix;
    
    #[inline]
    fn sub(self, other: Pix) -> Pix {
        Pix {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl SubAssign for Pix {
    #[inline]
    fn sub_assign(&mut self, other: Pix) {
        self.r -= other.r;
        self.g -= other.g;
        self.b -= other.b;
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Pix>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let mut pixels: Vec<Vec<Pix>> = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Pix::new());
            }
            pixels.push(row);
        }
        Image { width, height, pixels }
    }

    pub fn get_p3(&self) -> String {
        let mut s = self.get_p3_header();
        s.push_str(self.pix_string().as_str());
        return s;
    }

    fn get_p3_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }
    // output the pixel values
    // each 
    fn pix_string(&self) -> String {
        let mut s = String::new();
        for row in &self.pixels {
            for pix in row {
                s.push_str(pix.to_string().as_str());
            }
        }
        return s;
    }
}

// indexing operator
// return a reference to the pixel at the given row
impl Index<usize> for Image {
    type Output = Vec<Pix>;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.pixels[index]
    }
}

// indexing operator
// return a mutable reference to the pixel at the given row
impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

pub fn write_p3_file(filename: &str, image: &Image) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(image.get_p3().as_bytes()).expect("Unable to write data");
}