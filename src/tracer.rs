use super::basic::*;
use super::mesh::*;
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

pub struct Camera {
    pub origin: Vec3,
    pub screen: Screen,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn gradient_color(&self) -> Vec3 {
        let mut unit_direction: Vec3 = self.direction.clone();
        unit_direction.to_unit_len();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub t_min: f32,
    pub t_max: f32,
}

impl Interval {
    pub fn new() -> Interval {
        Interval { t_min: 0.0, t_max: f32::MAX }
    }
    pub fn initialize(t_min: f32, t_max: f32) -> Interval {
        Interval { t_min, t_max }
    }
    pub fn is_contained(&self, t: f32) -> bool {
        t > self.t_min && t < self.t_max
    }
}


pub struct Screen {
    pub start_point: Vec3, // the center of the screen
    pub horizontal: Vec3, // the horizontal vector of the screen
    pub vertical: Vec3, // the vertical vector of the screen
}

impl Screen {
    pub fn new(w_len: f32, h_len: f32) -> Screen {
        let mut start_point = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = Vec3 { x: w_len, y: 0.0, z: 0.0 };
        let vertical = Vec3 { x: 0.0, y: h_len, z: 0.0 };
        start_point -= horizontal * 0.5;
        start_point += vertical * 0.5;
        Screen { start_point, horizontal, vertical }
    }

    pub fn gradient_render(&self, camera: Vec3, width: usize, height: usize, filename: &str) {
        let mut image = Image::new(width, height);
        for row in 0..height {
            for col in 0..width {
                let u = col as f32 / width as f32;
                let v = row as f32 / height as f32;
                let ray = Ray {
                    origin: camera,
                    direction: self.start_point - camera + self.horizontal * u - self.vertical * v,
                };
                let color = ray.gradient_color();
                image[row][col] = Pix::from_vec3(color);
            }
        }
        write_p3_file(filename, &image);
    }
}

pub struct Job {
    pub row: usize,
    pub col: usize,
    pub camera: Arc<Mutex<Camera>>,
    pub ray: Ray,
    pub interv: Interval,
    pub scene: Arc<Mesh>,
}

impl Job {
    pub fn new(
        row: usize,
        col: usize,
        camera: Arc<Mutex<Camera>>,
        ray: Ray,
        interv: Interval,
        scene: Arc<Mesh>,
    ) -> Job {
        Job { row, col, camera, ray, interv, scene }
    }

    // part where new job are created based on the current job
    pub fn update(&mut self, new_ray: Ray, new_interv: Interval){
        self.ray = new_ray;
        self.interv = new_interv;
    }
}

pub fn do_job(job: Job) -> Option<(Hitrecord, Job)> {
    let mut camera = job.camera.lock().unwrap();
    let cur_scene = job.scene.clone();
    let mut job_res = cur_scene.is_hit(&job.ray, &job.interv);
    
    None
}