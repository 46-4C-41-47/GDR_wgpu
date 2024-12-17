use std::{iter, sync::Arc};
use wgpu::{Adapter, CommandEncoder, Surface, TextureFormat, TextureView};
use winit::{
  dpi::PhysicalSize, 
  event::WindowEvent, 
  keyboard::{KeyCode, PhysicalKey}, 
  window::Window
};
use pollster::FutureExt as _;


pub struct WindowSate {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  //window: Arc<Window>
}


impl WindowSate {
  pub fn new(window: Arc<Window>) -> WindowSate {
    let size: PhysicalSize<u32> = window.inner_size();

    let instance: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });

    let surface: Surface = instance.create_surface(window.clone()).unwrap();

    let adapter: Adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      }).block_on().unwrap();

    let (device, queue) = Self::get_device_and_queue(&adapter);

    let surface_caps: wgpu::SurfaceCapabilities = surface.get_capabilities(&adapter);
    let surface_format: TextureFormat = surface_caps
      .formats
      .iter()
      .copied()
      .find(|f| f.is_srgb())
      .unwrap_or(surface_caps.formats[0]);
    let config: wgpu::SurfaceConfiguration = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      desired_maximum_frame_latency: 2,
      view_formats: vec![],
    };

    Self {
      surface,
      device,
      queue,
      config,
      size,
      //window
    }
  }


  pub fn render(&self, default_bg_color: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(default_bg_color),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
      });
    }

    self.queue.submit(iter::once(encoder.finish()));
    output.present();

    Ok(())
  }


  pub fn update(&mut self) {
    todo!()
  } 


  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }


  pub fn process_input(&mut self, event: &WindowEvent) -> bool {
    match event {
      // event::WindowEvent::KeyboardInput { 
      //   event: event::KeyEvent { 
      //     state: event::ElementState::Pressed, 
      //     physical_key: PhysicalKey::Code(KeyCode::F1), 
      //     .. 
      //   }, 
      //   .. 
      // } => {},

      _ => return false
    }
    return true;
  }


  fn get_device_and_queue(adapter: &Adapter) -> (wgpu::Device, wgpu::Queue) {
    let device_descriptor: wgpu::DeviceDescriptor = wgpu::DeviceDescriptor {
      label: None,
          required_features: wgpu::Features::empty(),
          required_limits: wgpu::Limits::default(),
          memory_hints: Default::default(),
    };

    adapter.request_device(&device_descriptor, None).block_on().unwrap()
  }
}
