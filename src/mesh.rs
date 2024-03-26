use std::default::Default;
use std::f32::EPSILON;
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
use super::tracer::*;
use super::basic::*;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(f32), // albedo
    Metal(f32, f32), // albedo, fuzziness
    Dielectric(f32), // refraction index
}

impl Default for Material {
    fn default() -> Material {
        Material::Lambertian(0.0) // default material is black body
    }
}

pub trait Hitable {
    fn is_hit(&self, ray: &Ray, t_range: &Interval) -> Option<Hitrecord>;
}

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        let e1 = b - a;
        let e2 = c - a;
        let mut normal = cross(&e1, &e2); // right-handed coordinate system
        normal.to_unit_len();
        Triangle { a, b, c, normal, material: Material::default()}
    }

    pub fn default() -> Triangle {
        let a = Default::default();
        let b = Default::default();
        let c = Default::default();
        let normal = Default::default();
        Triangle { a, b, c, normal, material: Material::Lambertian(0.0) }
    }
}

impl Hitable for Triangle {
    fn is_hit(&self, ray: &Ray, t_range: &Interval) -> Option<Hitrecord> {
        // Moller-Trumbore algorith
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let ray_cross_e2 = cross(&ray.direction, &e2);
        let det = dot(&e1, &ray_cross_e2);
        if det > -EPSILON && det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - self.a;
        let u = dot(&s, &ray_cross_e2) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = cross(&s, &e1);
        let v = dot(&ray.direction, &s_cross_e1) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        
        let t = dot(&e2, &s_cross_e1) * inv_det;
        if t_range.is_contained(t) {
            let mut h = Hitrecord {
                t: 0.0,
                p: Vec3::new(),
                normal: Vec3::new(),
                front_face: false,
                material: self.material,
            };
            h.t = t;
            h.p = ray.at(t);
            h.normal = self.normal;
            h.front_face = dot(&ray.direction, &self.normal) < 0.0;
            return Some(h);
        } else {
            return None;
        }
    }
}

impl Add<Vec3> for Triangle {
    type Output = Triangle;

    #[inline]
    fn add(self, other: Vec3) -> Triangle {
        Triangle {
            a: self.a + other,
            b: self.b + other,
            c: self.c + other,
            normal: self.normal,
            material: self.material,
        }
    }
}

impl AddAssign<Vec3> for Triangle {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.a += other;
        self.b += other;
        self.c += other;
    }
}

impl Sub<Vec3> for Triangle {
    type Output = Triangle;

    #[inline]
    fn sub(self, other: Vec3) -> Triangle {
        Triangle {
            a: self.a - other,
            b: self.b - other,
            c: self.c - other,
            normal: self.normal,
            material: self.material,
        }
    }
}

impl SubAssign<Vec3> for Triangle {
    #[inline]
    fn sub_assign(&mut self, other: Vec3) {
        self.a -= other;
        self.b -= other;
        self.c -= other;
    }
}

impl Mul<f32> for Triangle {
    type Output = Triangle;

    #[inline]
    fn mul(self, scalar: f32) -> Triangle {
        Triangle {
            a: self.a * scalar,
            b: self.b * scalar,
            c: self.c * scalar,
            normal: self.normal,
            material: self.material,
        }
    }
}

impl MulAssign<f32> for Triangle {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.a *= scalar;
        self.b *= scalar;
        self.c *= scalar;
    }
}

pub struct Hitrecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
}

// impl Hitrecord {
//     pub fn new() -> Hitrecord {
//         Hitrecord {
//             t: 0.0,
//             p: Vec3::new(),
//             normal: Vec3::new(),
//             front_face: false,
//             material: self.material,
//         }
//     }
// }

pub struct Mesh {
    pub hitable_list: Vec<Box<dyn Hitable>>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh { hitable_list: Vec::new() }
    }

    pub fn add_triangle(&mut self, triangle: Box<dyn Hitable>) {
        self.hitable_list.push(triangle);
    }
}

impl Hitable for Mesh {
    fn is_hit(&self, ray: &Ray, t_range: &Interval) -> Option<Hitrecord> {
        let mut closest_t: f32 = t_range.t_max;
        let mut hit: Option<Hitrecord> = None;
        for s in self.hitable_list.iter() {
            if let Some(h) = s.is_hit(ray, t_range) {
                if h.t < closest_t {
                    closest_t = h.t;
                    hit = Some(h);
                }
            }
        }
        return hit;
    }
}
