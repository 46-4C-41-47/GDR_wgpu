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



struct App<'a> {
  initialized: bool,
  fields: Option<Fields<'a>>,
  window: Option<Window>,
}


struct Fields<'a> {
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  surface: wgpu::Surface<'a>,
  window: &'a Window
}


impl<'a> App<'a> {
  pub fn new() -> Self {
    App { 
      initialized: false,
      fields: None,
      window: None
    }
  }


  fn init(&mut self, window: &'a Window) {
    let size: PhysicalSize<u32> = window.inner_size();

    let instance: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      ..Default::default()
    });

    let surface: wgpu::Surface<'a> = instance.create_surface(window).unwrap();

    let adapter: wgpu::Adapter = instance
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

    self.fields = Some(Fields {
      device,
      queue,
      config,
      size,
      surface,
      window
    });
    self.initialized = true;
  }


  fn get_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    let device_descriptor: wgpu::DeviceDescriptor = wgpu::DeviceDescriptor {
      label: None,
          required_features: wgpu::Features::empty(),
          required_limits: wgpu::Limits::default(),
          memory_hints: Default::default(),
    };

    adapter.request_device(&device_descriptor, None).block_on().unwrap()
  }


  fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    if !self.initialized || self.window.is_none() {
      self.init(self.window.as_ref().unwrap());
    }

    let output: wgpu::SurfaceTexture = self.fields.as_ref().unwrap().surface.get_current_texture()?;
    let view:TextureView = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder: CommandEncoder = self
      .fields.as_ref().unwrap().device
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

    self.fields.as_ref().unwrap().queue.submit(iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}


impl ApplicationHandler for App<'_> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let window_attributes = Window::default_attributes()
      .with_title("My Window")
      .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0));
    let window = event_loop.create_window(window_attributes).unwrap();
    self.window = Some(window);
  }

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: event::WindowEvent,
  ) {
    if window_id != self.window.as_ref().unwrap().id() {
      return;
    }

    match event {
      event::WindowEvent::CloseRequested | event::WindowEvent::KeyboardInput { 
        event: event::KeyEvent { 
          state: event::ElementState::Pressed, 
          physical_key: PhysicalKey::Code(KeyCode::Escape), 
          .. 
        }, 
        .. 
      } => {
        println!("exiting window");
        event_loop.exit();
      },

      WindowEvent::RedrawRequested => {
        self.window.as_ref().unwrap().request_redraw();
        self.render();
      },

      _ => (),  
    }
  }
}


fn main() {
  println!("app is running");
  let my_event_loop: EventLoop<()> = EventLoop::new().unwrap();
  
  let mut app = App::new();

  my_event_loop.run_app(&mut app).unwrap();
}
