use crate::tracer::*;
use crate::basic::*;
use crate::mesh::*;

#[cfg(test)]
pub mod unittests {
    use super::*;

    #[test]
    fn test_vec3_0() {
        let mut v: Vec3 = Vec3::new();
        v.set(1.0, 0.0, 0.0);
        assert_eq!(v.len(), 1.0);
    }

    #[test]
    fn test_vec3_1() {
        let mut v: Vec3 = Vec3::new();
        v.set(1.0, 1.0, 1.0);
        v.to_unit_len();
        debug_assert!(v.len() - 1.0 < 1e-6);
    }

    #[test]
    fn test_vec3_2() {
        let v1: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let v2: Vec3 = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
        assert_eq!(dot(&v1, &v2), -1.0);
        let v3: Vec3 = v1 + v2;
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, 0.0);
    }

    #[test]
    fn test_vec3_3() {
        let mut v1: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let v2: Vec3 = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
        v1 += v2;
        assert_eq!(v1.x, 0.0);
        assert_eq!(v1.y, 0.0);
        assert_eq!(v1.z, 0.0);
    }

    #[test]
    fn test_vec3_4() {
        let mut v1: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let v2: Vec3 = Vec3 { x: -1.0, y: 0.0, z: 0.0 };
        v1 -= v2;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 0.0);
        assert_eq!(v1.z, 0.0);
    }

    #[test]
    fn test_vec3_5() {
        let v1: Vec3 = Vec3::new();
        let v2: Vec3 = Vec3::new();
        let v3: Vec3 = v1 + v2;
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 0.0);
        assert_eq!(v3.z, 0.0);
    }


    #[test]
    fn test_pix_0() {
        let mut pix = Pix::new();
        pix.set(0u8, 255u8, 255u8);
        assert_eq!(pix.to_string(), "0 255 255\n");
    }

    #[test]
    fn test_pix_1() {
        let mut pix = Pix::new();
        pix.set_float(0.0, 1.0, 1.0);
        assert_eq!(pix.to_string(), "0 255 255\n");
    }

    #[test]
    fn test_pix_2() {
        let mut pix = Pix::new();
        pix.set(0u8, 255u8, 255u8);
        let pix2 = Pix { r: 255u8, g: 0u8, b: 0u8};
        let pix3 = pix + pix2;
        assert_eq!(pix3.to_string(), "255 255 255\n");
    }

    #[test]
    fn test_image_0() {
        let image = Image::new(2, 2);
        assert_eq!(image.get_p3(), "P3\n2 2\n255\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n");
    }

    #[test]
    fn test_image_1() {
        let mut image = Image::new(2, 2);
        image[0][0].set(0u8, 255u8, 255u8);
        image[0][1].set(255u8, 0u8, 255u8);
        image[1][0].set(255u8, 255u8, 0u8);
        image[1][1].set(255u8, 255u8, 255u8);
        assert_eq!(image.get_p3(), "P3\n2 2\n255\n0 255 255\n255 0 255\n255 255 0\n255 255 255\n");
    }

    #[test]
    fn test_write_p3_file() {
        let height: usize = 100;
        let width: usize = 200;
        let mut image = Image::new(width, height);
        for row in 0..height {
            for col in 0..width {
                let r = row as f32 / height as f32;
                let g = col as f32 / width as f32;
                let b = 0.2;
                image[row][col].set_float(r, g, b);
            }
        }
        write_p3_file("./tests/test_write_p3_file.ppm", &image);
    }

    #[test]
    fn test_ray_0() {
        let origin: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let direction: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let ray: Ray = Ray { origin, direction };
        let point: Vec3 = ray.at(2.0);
        assert_eq!(point.x, 2.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }

    #[test]
    fn test_screen_0() {
        let screen: Screen = Screen::new(4.0, 2.0);
        assert_eq!(screen.start_point.x, -2.0);
        assert_eq!(screen.start_point.y, 1.0);
        assert_eq!(screen.start_point.z, 0.0);
    }

    #[test]
    fn test_screen_1() {
        let screen: Screen = Screen::new(4.0, 2.0);
        let camera: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        screen.gradient_render(
            camera,
            200,
            100,
            "./tests/test_screen_1.ppm"
        );
    }

    // #[test]
    // #[ignore]
    // fn test_screen_2() {
    //     let screen: Screen = Screen::new(4.0, 2.0);
    //     let camera: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
    //     let center: Vec3 = Vec3 { x: 0.2, y: 0.2, z: -1.5 };
    //     screen.icosahedron_render(
    //         camera,
    //         center,
    //         400,
    //         200,
    //         "./tests/test_screen_2.ppm"
    //     );
    // }

    // #[test]
    // #[ignore]
    // fn test_screen_3() {
    //     let screen: Screen = Screen::new(4.0, 2.0);
    //     let camera: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
    //     let center: Vec3 = Vec3 { x: 0.2, y: 0.2, z: -1.5 };
    //     screen.sphere_render(
    //         camera,
    //         center,
    //         1.0,
    //         200,
    //         100,
    //         "./tests/test_screen_3.ppm"
    //     );
    //     screen.sphere_render_antialias(
    //         camera,
    //         10,
    //         center,
    //         1.0,
    //         200,
    //         100,
    //         "./tests/test_screen_4.ppm"
    //     );
    // }

    #[test]
    fn test_triangle_is_hit_0() {
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        };
        let triangle = Triangle::new(
            Vec3 { x: 0.0, y: 0.0, z: 1.0 },
            Vec3 { x: 1.0, y: 0.0, z: 1.0 },
            Vec3 { x: 0.0, y: 1.0, z: 1.0 },
        );
        let interv = Interval::new();
        assert_eq!(triangle.is_hit(&ray, &interv).unwrap().t, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_triangle_is_hit_1() {
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        };
        let triangle = Triangle::new(
            Vec3 { x: -1.0, y: -1.0, z: 1.0 },
            Vec3 { x: -1.0, y: 0.0, z: 1.0 },
            Vec3 { x: 0.0, y: -1.0, z: 1.0 },
        );
        let interv = Interval::new();
        triangle.is_hit(&ray, &interv).unwrap();
    }

    #[test]
    fn test_triangle_is_hit_2() {
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        };
        let triangle = Triangle::new(
            Vec3 { x: -1.0, y: 1.0, z: 1.0 },
            Vec3 { x: 1.0, y: 1.0, z: 1.0 },
            Vec3 { x: 0.0, y: -2.0, z: 1.0 },
        );
        let interv = Interval::new();
        assert_eq!(triangle.is_hit(&ray, &interv).unwrap().t, 1.0);
    }

    #[test]
    fn test_mesh_0() {
        let mut mesh = Mesh::new();
        let t1 = Triangle::new(
            Vec3 { x: 0.0, y: 0.0, z: 1.0 },
            Vec3 { x: 1.0, y: 0.0, z: 1.0 },
            Vec3 { x: 0.0, y: 1.0, z: 1.0 },
        );
        let t2 = Triangle::new(
            Vec3 { x: 0.0, y: 0.0, z: 2.0 },
            Vec3 { x: 1.0, y: 0.0, z: 2.0 },
            Vec3 { x: 0.0, y: 1.0, z: 2.0 },
        );
        mesh.add_triangle(Box::new(t1));
        mesh.add_triangle(Box::new(t2));
        let ray = Ray {
            origin: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        };
        let interv = Interval::new();
        assert_eq!(mesh.is_hit(&ray, &interv).unwrap().t, 1.0);
    }
}