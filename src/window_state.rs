use std::iter;
use wgpu::{CommandEncoder, Surface, TextureFormat, TextureView};
use winit::{
  application::ApplicationHandler, 
  dpi::PhysicalSize, 
  event::{self, WindowEvent}, 
  event_loop::{self, EventLoop}, 
  keyboard::{KeyCode, PhysicalKey}, 
  window::{self, Window}
};
use pollster::FutureExt as _;


pub struct WindowSate<'a> {
  surface: wgpu::Surface<'a>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
}


impl<'a> WindowSate<'a> {
  pub fn new<'b>(window: &'static Window) -> WindowSate<'b> {
    todo!()
  }


  fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    todo!()
  }


  fn input(&mut self, event: &WindowEvent) -> bool {
    todo!()
  }


  fn update(&mut self) {
    todo!()
  } 


  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    todo!()
  }
}
