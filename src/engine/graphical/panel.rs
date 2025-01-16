use std::panic;
use wgpu::RenderPass;
use crate::engine::geometry::{FSize, USize, Point2D};
use super::texture::Texture;



pub struct PanelDescriptor {
  path_to_texture: &'static str,
  position: Point2D,
  width: u32,
  height: u32,

  texture_position: Point2D,
  texture_width: f32,
  texture_height: f32
}


pub struct Panel {
  texture: Texture,
  
  pub position: Point2D,
  pub size: USize,

  texture_position: Point2D,
  texture_size: FSize,
}

impl Panel {
  pub fn new(descriptor: &PanelDescriptor, device: &wgpu::Device, queue: &wgpu::Queue) -> Self { 
    Self {
      texture: Texture::new(descriptor.path_to_texture, device, queue),
      position: descriptor.position,
      size: USize{ width: descriptor.width, height: descriptor.height },
      texture_position: descriptor.texture_position,
      texture_size: FSize{ width: descriptor.texture_width, height: descriptor.texture_height }
    } 
  }


  pub fn set_texture_slice_position(&mut self, position: Point2D) { 
    if position.x < 0.0 || 1.0 < position.x || position.y < 0.0 || 1.0 < position.y { 
      panic!("Texture position should be between 0.0 and 1.0"); 
    }
    self.texture_position = position;
  }


  pub fn set_texture_slice_size(&mut self, size: FSize) { 
    if size.width < 0.0 || 1.0 < size.width || size.height < 0.0 || 1.0 < size.height { 
      panic!("Texture size should be between 0.0 and 1.0"); 
    }
    self.texture_size = size;
  }


  fn draw(&mut self, render_pass: RenderPass<'_>) {
    render_pass.set_pipeline(&self.render_pipeline);
    render_pass.set_bind_group(0, self.texture.get_bind_group(), &[]);
    render_pass.set_bind_group(1, &self.camera.get_bind_group(), &[]);
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass.draw(0..self.num_vertices, 0..1);
  }
}
