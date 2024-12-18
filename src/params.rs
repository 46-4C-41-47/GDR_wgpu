pub mod graphical {
  pub const SHADER_PATH: &str = "../res/shaders/shader.wgsl";
  pub const TEXTURE_PATH: &str = "../res/textures/texture.png";
  pub const BACKGROUND_COLOR: wgpu::Color = wgpu::Color { r: 0.02, g: 0.02, b: 0.04, a: 1.0 };
}


pub enum ControllerInput {
  UP,
  DOWN,
  LEFT,
  RIGHT,

  DIAGONAL_UP_LEFT,
  DIAGONAL_UP_RIGHT,
  DIAGONAL_DOWN_LEFT,
  DIAGONAL_DOWN_RIGHT,
  
  L_PUNCH,
  M_PUNCH,
  H_PUNCH,

  L_KICK,
  M_KICK,
  H_KICK,
}
