pub const QUAD: &[Vertex] = &[
  Vertex { position: [-0.5, -0.5,  0.0], texture_coords: [0.0, 1.0] },
  Vertex { position: [-0.5,  0.5,  0.0], texture_coords: [0.0, 0.0] },
  Vertex { position: [ 0.5, -0.5,  0.0], texture_coords: [1.0, 1.0] },
  
  Vertex { position: [-0.5,  0.5,  0.0], texture_coords: [0.0, 0.0] },
  Vertex { position: [ 0.5,  0.5,  0.0], texture_coords: [1.0, 0.0] },
  Vertex { position: [ 0.5, -0.5,  0.0], texture_coords: [1.0, 1.0] },
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  position: [f32; 3],
  texture_coords: [f32; 2],
}


impl Vertex {
  pub fn get_descriptor() -> wgpu::VertexBufferLayout<'static> { 
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
          offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x2,
        }
      ]
    }
  }
}
