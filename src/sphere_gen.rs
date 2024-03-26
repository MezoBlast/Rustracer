pub struct Icosahedron {
    pub mesh: Mesh,
}

impl Icosahedron {
    pub fn new(center: Vec3) -> Icosahedron {
        let mut vertices: Vec<Vec3> = Vec::new();
        let phi = (1.0 + 5.0f32.sqrt()) / 2.0;
        let a = 1.0f32;
        let b = 1.0f32/phi;
        vertices.push(center.clone());
        vertices.push(Vec3 {x: 0., y: b, z: -a});
        vertices.push(Vec3 {x: b, y: a, z: 0.});
        vertices.push(Vec3 {x: -b, y: a, z: 0.});
        vertices.push(Vec3 {x: 0., y: b, z: a});
        vertices.push(Vec3 {x: 0., y: -b, z: a});
        vertices.push(Vec3 {x: -a, y: 0., z: b});
        vertices.push(Vec3 {x: 0., y: -b, z: -a});
        vertices.push(Vec3 {x: a, y: 0., z: -b});
        vertices.push(Vec3 {x: a, y: 0., z: b});
        vertices.push(Vec3 {x: -a, y: 0., z: -b});
        vertices.push(Vec3 {x: b, y: -a, z: 0.});
        vertices.push(Vec3 {x: -b, y: -a, z: 0.});
        
        for v in vertices.iter_mut() {
            v.to_unit_len();
        }
        for v in vertices.iter_mut() {
            *v += center;
        }

        let mut mesh: Mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(vertices[3], vertices[2], vertices[1]));
        mesh.add_triangle(Triangle::new(vertices[2], vertices[3], vertices[4]));
        mesh.add_triangle(Triangle::new(vertices[6], vertices[5], vertices[4]));
        mesh.add_triangle(Triangle::new(vertices[5], vertices[9], vertices[4]));
        mesh.add_triangle(Triangle::new(vertices[8], vertices[7], vertices[1]));
        mesh.add_triangle(Triangle::new(vertices[7], vertices[10], vertices[1]));
        mesh.add_triangle(Triangle::new(vertices[12], vertices[11], vertices[5]));
        mesh.add_triangle(Triangle::new(vertices[11], vertices[12], vertices[7]));
        mesh.add_triangle(Triangle::new(vertices[10], vertices[6], vertices[3]));
        mesh.add_triangle(Triangle::new(vertices[6], vertices[10], vertices[12]));
        mesh.add_triangle(Triangle::new(vertices[9], vertices[8], vertices[2]));
        mesh.add_triangle(Triangle::new(vertices[8], vertices[9], vertices[11]));
        mesh.add_triangle(Triangle::new(vertices[3], vertices[6], vertices[4]));
        mesh.add_triangle(Triangle::new(vertices[9], vertices[2], vertices[4]));
        mesh.add_triangle(Triangle::new(vertices[10], vertices[3], vertices[1]));
        mesh.add_triangle(Triangle::new(vertices[2], vertices[8], vertices[1]));
        mesh.add_triangle(Triangle::new(vertices[12], vertices[10], vertices[7]));
        mesh.add_triangle(Triangle::new(vertices[8], vertices[11], vertices[7]));
        mesh.add_triangle(Triangle::new(vertices[6], vertices[12], vertices[5]));
        mesh.add_triangle(Triangle::new(vertices[11], vertices[9], vertices[5]));
    
        Icosahedron { mesh }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mesh: Mesh,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        let ico = Icosahedron::new(Vec3::new());
        let mut mesh = ico.mesh;
        let n: i32 = 3;
        for _ in 0..n {
            mesh = tri_segment_mesh(mesh);
        }
        for triangle in mesh.triangles.iter_mut() {
            *triangle *= radius; // this is stupid, why Rust do this dereferemce?
            *triangle += center;
        }
        Sphere { center, radius, mesh }
    }
}

fn tri_segment_mesh(mesh: Mesh) -> Mesh {
    let mut new_mesh = Mesh::new();
    for triangle in mesh.triangles.iter() {
        let a = triangle.a;
        let b = triangle.b;
        let c = triangle.c;
        let mut ab = (a + b) * 0.5;
        let mut bc = (b + c) * 0.5;
        let mut ca = (c + a) * 0.5;
        ab.to_unit_len();
        bc.to_unit_len();
        ca.to_unit_len();
        new_mesh.add_triangle(Triangle::new(a, ab, ca));
        new_mesh.add_triangle(Triangle::new(ab, b, bc));
        new_mesh.add_triangle(Triangle::new(bc, c, ca));
        new_mesh.add_triangle(Triangle::new(ab, bc, ca));
    }
    return new_mesh;
}