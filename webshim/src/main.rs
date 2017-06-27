extern crate mesh;

pub use mesh::*;

fn main() {
    println!("Hello from webshim.");
    let triangle = Mesh::triangle();
    println!("triangle: {:?}", triangle.positions);
}
