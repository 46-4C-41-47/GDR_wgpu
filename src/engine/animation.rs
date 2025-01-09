pub struct AnimationDescriptor {
  pub image_size: (u32, u32),
  pub frames_duration: Vec<u32>,
  pub path_to_file: String,
}


impl AnimationDescriptor {
  pub fn from_json(json: String) -> Self { todo!() }
}


pub struct GraphicConfig<'a> {
  pub pipeline: &'a wgpu::RenderPipeline,
  pub device: &'a wgpu::Device,
  pub queue: &'a wgpu::Queue,
}


pub struct Animation {
  frame_count: u32,
  frame_index: u32,
  graphic_config: GraphicConfig<'static>,
}


impl Animation {
  pub fn new(config: AnimationDescriptor, graphic_config: GraphicConfig<'static>) -> Self {
    Self {
      frame_count: config.frames_duration.iter().sum::<u32>(),
      frame_index: 0,
      graphic_config,
    } 
  }


  pub fn draw_next_frame(&mut self) { todo!() }
}
