use std::os::raw::c_char;
use std::ffi::CString;
use std::collections::HashMap;

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
    pub x: u32,
    pub positions: Vec<Vector3>,
}

impl Mesh {
    pub fn triangle() -> Mesh {
        let mut mesh = Mesh { x: 42, positions: Vec::new() };
        mesh.positions.push(Vector3 { x:  0.0, y: 5.0, z: 0.0 });
        mesh.positions.push(Vector3 { x: -2.5, y: 0.0, z: 0.0 });
        mesh.positions.push(Vector3 { x:  2.5, y: 0.0, z: 0.0 });
        mesh
    }
}

#[no_mangle]
pub fn my_add(a: f32, b: f32) -> f32 {
    a + b
}

#[no_mangle]
pub fn get_data() -> *mut c_char {
    let mut data = HashMap::new();
    data.insert("Alice", "send");
    data.insert("Bob", "recieve");
    data.insert("Carol", "intercept");

    let descriptions = data.iter()
        .map(|(p,a)| format!("{} likes to {} messages", p, a))
        .collect::<Vec<_>>();

    CString::new(descriptions.join(", "))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub fn mesh_create_triangle() -> Box<Mesh> {
    Box::new(Mesh::triangle())
}

#[no_mangle]
pub fn mesh_get_positions(mesh: &Mesh) -> &[Vector3] {
    mesh.positions.as_slice()
}
