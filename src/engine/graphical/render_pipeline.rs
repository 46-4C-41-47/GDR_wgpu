use std::{collections::HashMap, io::Read};

static mut INSTANCE: Option<RenderPipelineManager> = None;



pub struct RenderPipelineDescriptor<'a> {
  pub pipeline_name: &'static str,
  pub shader_path: &'static str,
  pub bind_group_layouts: &'a [&'a wgpu::BindGroupLayout]
}



pub struct RenderPipelineManager {
  default_pipeline: wgpu::RenderPipeline,
  pipelines: HashMap<String, wgpu::RenderPipeline>,
}

impl RenderPipelineManager {
  const DEFAULT_PIPELINE_DESCRIPTOR: RenderPipelineDescriptor<'_> = RenderPipelineDescriptor {
    pipeline_name: "default",
    shader_path: "../res/shaders/shader.wgsl",
    bind_group_layouts: &[],
  };


  pub fn create_instance(device: &wgpu::Device) {
    unsafe {
      if Self::is_initialized() { return; }

      INSTANCE = Some(RenderPipelineManager {
        default_pipeline: Self::build_pipeline(device, &Self::DEFAULT_PIPELINE_DESCRIPTOR),
        pipelines: HashMap::new(),
      });
    }
  }


  pub fn is_initialized() -> bool { unsafe { INSTANCE.is_some() } }


  pub fn create(device: &wgpu::Device, descriptor: &RenderPipelineDescriptor) {
    unsafe{ 
      INSTANCE.as_mut().unwrap().pipelines.insert(
        descriptor.pipeline_name.to_string(), 
        Self::build_pipeline(device, descriptor)
      ); 
    }
  }


  pub fn get_default() -> &'static wgpu::RenderPipeline { 
    unsafe { 
      Self::init_check("RenderPipelineManager must be created before getting default pipeline");
      &INSTANCE.as_ref().unwrap().default_pipeline 
    } 
  }


  pub fn get(pipeline_name: String) -> Option<&'static wgpu::RenderPipeline> {
    unsafe { 
      Self::init_check("RenderPipelineManager must be created before getting pipeline");
      INSTANCE.as_ref().unwrap().pipelines.get(&pipeline_name) 
    }
  }


  fn build_pipeline(device: &wgpu::Device, descriptor: &RenderPipelineDescriptor) -> wgpu::RenderPipeline {
    let shader: wgpu::ShaderModule = Self::get_shader_module(device, descriptor.shader_path);
    let render_pipeline_layout: wgpu::PipelineLayout = device.create_pipeline_layout(
      &wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: descriptor.bind_group_layouts,
        push_constant_ranges: &[],
      });

    return device.create_render_pipeline(
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
      }
    );
  }


  fn get_shader_module(device: &wgpu::Device, shader_path: &str) -> wgpu::ShaderModule {
    let mut shader_file: std::fs::File = std::fs::File::open(shader_path).unwrap();
    let mut shader_string: String = String::new();
    shader_file.read_to_string(&mut shader_string).unwrap();

    let shader_module_descriptor: wgpu::ShaderModuleDescriptor = wgpu::ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(shader_string.as_str().into()),
    };
    
    return device.create_shader_module(shader_module_descriptor);
  }


  fn init_check(message: &str) {
    unsafe {
      if INSTANCE.is_none() {
        panic!("{}", message);
      }
    }
  }
}
