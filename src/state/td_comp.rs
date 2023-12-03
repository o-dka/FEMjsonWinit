use bytemuck::{Zeroable,Pod};
use serde::{Serialize,Deserialize};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable,Serialize,Deserialize)]
pub struct Vertex {
  position: [f32; 4],
  color: [f32; 4],
}
#[derive(Serialize, Deserialize)]
pub struct TdObject {
  pub vertices : Vec<Vertex>, // a vector of Vertexes with their color info 
  pub position : [f32;3] , // position of the object is it's central point (i think!)
}

impl TdObject {
 pub fn new () -> Self{
    let json_data: Vec<[i8;3]> = serde_json::from_str::<Vec<[i8;3]>>(include_str!("../../test.json")).unwrap();
    let vertices  = create_vertices(json_data.into());
    let position = (0.0,0.0,0.0).into(); 
    TdObject {vertices, position}
  }
  
}

fn vertex(p: [i8; 3], c: [i8; 3]) -> Vertex {
    Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
    }
}

fn create_vertices(pos_from_json : Vec<[i8;3]>) -> Vec<Vertex> {
    // let s : Deserializer<Vertex> ;
    let col: [i8;3] = [0, 0, 1];
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

