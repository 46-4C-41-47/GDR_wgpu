use super::gdr_engine::Input;
use super::hitbox::{FrameHitbox, Hitbox};


#[derive(Copy, Clone)]
pub struct CommandPattern();


pub struct Action {
  name: String,
  frame_count: u32,
  frame_index: u32,
  command: CommandPattern,
  hitboxes: Vec<FrameHitbox>,
}


impl Action {
  pub fn new(json: String) -> Self { 
    Self {
      name: String::from(""),
      frame_count: 1,
      frame_index: 1,
      command: CommandPattern(),
      hitboxes: Vec::new(),
    } 
  }


  pub fn next(&mut self) -> bool { todo!() }


  pub fn get_name(&self) -> &String { &self.name }

  pub fn get_hitbox(&self) -> &FrameHitbox { &self.hitboxes[self.frame_index as usize] }
  
  pub fn get_command(&self) -> &CommandPattern { &self.command }
}
