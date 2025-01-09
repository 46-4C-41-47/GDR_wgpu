use super::gdr_engine::Input;
use super::hitbox::Hitbox;


pub struct Action {
  frame_count: u32,
  frame_index: u32,
  command: Vec<Vec<Input>>,
  hitbox: Vec<Vec<Hitbox>>,
}


impl Action {
  pub fn new(json: String) -> Self { 
    Self {
      frame_count: 1,
      frame_index: 1,
      command: Vec::new(),
      hitbox: Vec::new(),
    } 
  }


  pub fn get_current_frame_hitbox(&self) -> Vec<Hitbox> { todo!() }


  pub fn next_frame(&mut self) { todo!() }
}
