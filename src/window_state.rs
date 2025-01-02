use std::{iter, sync::Arc};

use wgpu::{
  util::DeviceExt, Adapter, BindGroupLayout, CommandEncoder, RenderPass, Surface, SurfaceTexture, TextureFormat, TextureView
};
use winit::{
  dpi::PhysicalSize, 
  event::WindowEvent, 
  keyboard::{ KeyCode, PhysicalKey }, 
  window::Window
};
use pollster::FutureExt as _;

use crate::{
  texture::{self, Texture}, 
  vertex::{self, Vertex, QUAD}, 
  camera::{ self, Camera }
};
use crate::params::graphical;


pub struct WindowSate {
  surface: wgpu::Surface<'static>,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  size: winit::dpi::PhysicalSize<u32>,
  render_pipeline: wgpu::RenderPipeline,
  vertex_buffer: wgpu::Buffer,
  num_vertices: u32,
  texture: Texture,
  camera: camera::Camera,
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

    let texture: Texture = Texture::new("./res/textures/standing_sagat.png", &device, &queue);
    let camera: Camera = Camera::new(&device);

    let render_pipeline: wgpu::RenderPipeline = Self::get_render_pipeline(
      &device, 
      &config, 
      &[texture.get_bind_group_layout(), camera.get_bind_group_layout()]
    );

    let vertex_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertex::QUAD),
        usage: wgpu::BufferUsages::VERTEX,
      }
    );

    Self {
      surface,
      device,
      queue,
      config,
      size,
      render_pipeline,
      vertex_buffer,
      num_vertices: QUAD.len() as u32,
      texture,
      camera,
    }
  }


  pub fn render(&self, default_bg_color: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
    let output: SurfaceTexture = self.surface.get_current_texture()?;
    let view: TextureView = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder: CommandEncoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let mut render_pass: RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
    
      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.set_bind_group(0, self.texture.get_bind_group(), &[]);
      render_pass.set_bind_group(1, &self.camera.get_bind_group(), &[]);
      render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
      render_pass.draw(0..self.num_vertices, 0..1);
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


  fn get_render_pipeline(
    device: &wgpu::Device, 
    config: &wgpu::SurfaceConfiguration, 
    bind_group_layout: &[&BindGroupLayout]
  ) -> wgpu::RenderPipeline { 
    let shader = device.create_shader_module(wgpu::include_wgsl!("../res/shaders/shader.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(
      &wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: bind_group_layout,
        push_constant_ranges: &[],
      });

    let render_pipeline = device.create_render_pipeline(
      &wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
          module: &shader,
          entry_point: "vs_main",
          buffers: &[Vertex::get_descriptor()],
          compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
          module: &shader,
          entry_point: "fs_main",
          targets: &[Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
          })],
          compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::TriangleList,
          strip_index_format: None,
          front_face: wgpu::FrontFace::Cw,
          cull_mode: Some(wgpu::Face::Back),
          polygon_mode: wgpu::PolygonMode::Fill,
          unclipped_depth: false,
          conservative: false,
        },
        depth_stencil: None, 
        multisample: wgpu::MultisampleState {
          count: 1,
          mask: !0,
          alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
      });

    return render_pipeline;
  }
}
