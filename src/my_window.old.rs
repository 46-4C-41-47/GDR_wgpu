use std::{default, iter};
use pollster::FutureExt as _;

use wgpu::{Adapter, RenderPass, Surface, SurfaceTexture, TextureFormat, TextureView};
use winit::{
  application::ApplicationHandler, 
  dpi::PhysicalSize, 
  event::*, 
  event_loop::{self, EventLoop}, 
  keyboard::{KeyCode, PhysicalKey}, 
  window::{self, Window}
};

/*
struct State<'a> {
  surface: wgpu::Surface<'a>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  window: &'a Window,
}

impl<'a> State<'a> {
  fn new(window: &'a Window) -> State<'a> {
    let size: PhysicalSize<u32> = window.inner_size();

    let instance: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });

    let surface: Surface = instance.create_surface(window).unwrap();

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

    return Self {
      surface,
      device,
      queue,
      config,
      size,
      window,
    };
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


  fn window(&self) -> &Window {
    &self.window
  }


  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }


  #[allow(unused_variables)]
  fn input(&mut self, event: &WindowEvent) -> bool {
    false
  }


  fn update(&mut self) {}


  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output: SurfaceTexture = self.surface.get_current_texture()?;
    let view: TextureView = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder: wgpu::CommandEncoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let _render_pass: RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            }),
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
}



struct MyWindow<'a> {
  surface: wgpu::Surface<'a>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  event_loop: EventLoop<()>,
  window: Window,
}


impl<'a> MyWindow<'a> {
  pub fn new() -> Self {
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
    let window: Window = WindowBuilder::new().build(&event_loop).unwrap();

    let size: PhysicalSize<u32> = window.inner_size();

    let instance: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });

    let surface: Surface = instance.create_surface(&window).unwrap();

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
      event_loop,
      window
    }
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


  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }


  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    let output: SurfaceTexture = self.surface.get_current_texture()?;
    let view: TextureView = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder: wgpu::CommandEncoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let _render_pass: RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: 0.1,
              g: 0.2,
              b: 0.3,
              a: 1.0,
            }),
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


  fn process_input(&self, event: &WindowEvent) -> bool {
    false
  }


  pub fn run(&self) {
    env_logger::init();

    let mut surface_configured = false;

    self.event_loop
      .run(move |event, control_flow| {
        match event {
          Event::WindowEvent {
            ref event,
            window_id,
          } if window_id == self.window.id() => {
            if !self.process_input(event) {
              // UPDATED!
              match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                  event:
                    KeyEvent {
                      state: ElementState::Pressed,
                      physical_key: PhysicalKey::Code(KeyCode::Escape),
                      ..
                    },
                  ..
                } => control_flow.exit(),
                
                WindowEvent::Resized(physical_size) => {
                  log::info!("physical_size: {physical_size:?}");
                  surface_configured = true;
                  self.resize(*physical_size);
                }
                
                WindowEvent::RedrawRequested => {
                  // This tells winit that we want another frame after this one
                  self.window.request_redraw();

                  if !surface_configured {
                    return;
                  }

                  //self.update();
                  match self.render() {
                    Ok(_) => {}
                    
                    // Reconfigure the surface if it's lost or outdated
                    Err(
                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                    ) => self.resize(self.size),
                    
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                      log::error!("OutOfMemory");
                      control_flow.exit();
                    }

                    // This happens when the a frame takes too long to present
                    Err(wgpu::SurfaceError::Timeout) => {
                      log::warn!("Surface timeout")
                    }
                  }
                }
                _ => {}
              }
            }
          }
          _ => {}
        }
      })
      .unwrap();
  }
}
  */

struct MyWindow {
  event_loop: EventLoop<()>,
}


impl MyWindow {
  pub fn new() -> Self {
    Self {
      event_loop: EventLoop::new().unwrap()
    }
  }


  pub fn run(&mut self) {
    //self.event_loop.run_app(self);
  }
}


impl ApplicationHandler for MyWindow {
  fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
    todo!()
  }

  fn window_event(
    &mut self,
    event_loop: &event_loop::ActiveEventLoop,
    window_id: window::WindowId,
    event: WindowEvent,
  ) {
    todo!()
  }
}

