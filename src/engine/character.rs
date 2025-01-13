use super::{
  hitbox::Hitbox, 
  input::{
    InputDevice, 
    InputFlow
  }
};


pub enum CharacterState {
  Normal,
  Stun,
  Grabbed,
  Falling,
  Preparing,
  Recovering,
  Dead,
}


pub struct Character {
  pub health: u32,
  pub super_: u32,
  pub stun: u32,

  defense: u32,
  attack: u32,
  input_flow: InputFlow
}


impl Character {
  pub fn new(input_device: InputDevice) -> Self {
    Self {
      health: 0,
      super_: 0,
      stun: 0,

      defense: 0,
      attack: 0,
      input_flow: InputFlow::new(input_device, Vec::new()),
    }
  }


  pub fn check_collisions(&mut self, enney_hitbox: Vec<Hitbox>) { 
    todo!() 
  }


  pub fn get_hitbox(&self) -> Vec<Hitbox> { todo!() }


  fn hit(&mut self) { todo!() }


  fn grabbed(&mut self) { todo!() }


  fn grabbing(&mut self) { todo!() }
}
