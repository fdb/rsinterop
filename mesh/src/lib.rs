#[repr(C)]
#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Mesh {
    pub positions: Vec<Vector3>,
}

impl Mesh {
    pub fn triangle() -> Mesh {
        let mut mesh = Mesh { positions: Vec::new() };
        mesh.positions.push(Vector3 { x:  0.0, y: 5.0, z: 0.0 });
        mesh.positions.push(Vector3 { x: -2.5, y: 0.0, z: 0.0 });
        mesh.positions.push(Vector3 { x:  2.5, y: 0.0, z: 0.0 });
        mesh
    }
}

#[no_mangle]
pub extern "C" fn mesh_create_triangle() -> Box<Mesh> {
    Box::new(Mesh::triangle())
}

#[no_mangle]
pub extern "C" fn mesh_get_positions(mesh: &Mesh) -> Box<&[Vector3]> {
    Box::new(mesh.positions.as_slice())
}
