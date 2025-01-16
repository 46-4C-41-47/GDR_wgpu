use std::collections::HashMap;

static mut INSTANCE: Option<RenderPipelineManager> = None;



pub struct RenderPipelineDescriptor<'a> {
  device: &'a wgpu::Device,
  queue: &'a wgpu::Queue,
  shader_path: &'static str
}



pub struct RenderPipelineManager {
  default_pipeline: wgpu::RenderPipeline,
  pipelines: HashMap<String, wgpu::RenderPipeline>,
}

impl RenderPipelineManager {
  pub fn create_instance(default_pipeline: &RenderPipelineDescriptor) {
    unsafe {
      if Self::is_initialized() { return; }

      INSTANCE = Some(RenderPipelineManager {
        default_pipeline: Self::build_pipeline(default_pipeline),
        pipelines: HashMap::new(),
      });
    }
  }


  pub fn is_initialized() -> bool { unsafe { INSTANCE.is_some() } }


  pub fn create(pipeline_name: &str, descriptor: &RenderPipelineDescriptor) {
    unsafe{ 
      INSTANCE.as_mut().unwrap().pipelines.insert(
        pipeline_name.to_string(), 
        Self::build_pipeline(descriptor)
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


  fn build_pipeline(descriptor: &RenderPipelineDescriptor) -> wgpu::RenderPipeline {
    todo!()
  }


  fn init_check(message: &str) {
    unsafe {
      if INSTANCE.is_none() {
        panic!("{}", message);
      }
    }
  }
}
