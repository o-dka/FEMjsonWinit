use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Serialize, Deserialize)]
pub struct TdObject {
    pub vertices: Vec<Vertex>, // a vector of a Vertex struct (see line 29 or just go to type def in vscode)
    pub indices : Vec<u16>      // indices of the vertices
    // TODO : material   should provide a union of material properties  (steel,reinforced concrete, etc.)
}
#[derive(Serialize, Deserialize)]
struct JsonIn {
    vers : Vec<[i8;3]>,
    inds : Vec<u16>
}

impl TdObject {
    pub fn new(filename : &str) -> Self {
        let  file_data =fs::read_to_string(filename) 
            .expect("FILE READ ERROR ");
        
        let json_data:JsonIn = serde_json::from_str(&file_data).unwrap();
        let indices: Vec<u16> = json_data.inds;
        let vertices = create_vertices(json_data.vers);
        TdObject { vertices, indices }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Serialize, Deserialize)]
pub struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

fn vertex(p: [i8; 3], c: [i8; 3]) -> Vertex {
    Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
    }
}

fn create_vertices(pos_from_json: Vec<[i8; 3]>) -> Vec<Vertex> {
    let col: [i8; 3] = [0, 0, 1];
    let mut data: Vec<Vertex> = Vec::with_capacity(pos_from_json.len());
    for i in pos_from_json {
        data.push(vertex(i, col));
    }
    data.to_vec()
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
