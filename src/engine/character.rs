// use super::{
//   hitbox::Hitbox, 
//   input::{
//     InputDevice, 
//     InputFlow
//   }
// };


// pub enum CharacterState {
//   Normal,
//   Stun,
//   Grabbed,
//   Falling,
//   Preparing,
//   Recovering,
//   Dead,
// }


// pub struct Character {
//   pub health: u32,
//   pub super_: u32,
//   pub stun: u32,

//   defense: u32,
//   attack: u32,
//   input_flow: InputFlow
// }


// impl Character {
//   pub fn new(input_device: InputDevice) -> Self {
//     Self {
//       health: 0,
//       super_: 0,
//       stun: 0,
//       defense: 0,
//       attack: 0,
//       input_flow: InputFlow::new(input_device, Vec::new()),
//     }
//   }
//   pub fn check_collisions(&mut self, enney_hitbox: Vec<Hitbox>) { 
//     todo!() 
//   }
//   pub fn get_hitbox(&self) -> Vec<Hitbox> { todo!() }
//   fn hit(&mut self) { todo!() }
//   fn grabbed(&mut self) { todo!() }
//   fn grabbing(&mut self) { todo!() }
// }

use super::{action::Action, input::{CommandInterpreter, InputDevice}};


pub struct Character<'a> {
  health: u32,
  actions: Vec<Action>,
  current_action: Option<&'a mut Action>,
  command_interpreter: CommandInterpreter,
}


impl Character<'_> {
  const DEFAULT_ACTION: Action = Action::new(String::from("Idle"));


  pub fn new() -> Self {
    //let actions = Vec::new();

    Self {
      health: 0,
      actions: Vec::new(),
      current_action: None,
      command_interpreter: CommandInterpreter::new(InputDevice(), Vec::new()),
    }
  }


  pub fn next(&mut self) {
    if self.current_action.is_none() || !self.current_action.unwrap().next() {
      self.current_action = self.get_action("Idle");
    }
  }


  fn get_action(&self, name: &str) -> Option<& mut Action> { 
    Some(self.actions.iter().find(|action| action.get_name() == name).unwrap()) 
  }
}
