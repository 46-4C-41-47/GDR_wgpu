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

use super::{action::{self, ActionSet, CommandPattern}, input::{CommandInterpreter, InputDevice}};


struct Point2D {
  pub x: f32,
  pub y: f32,
}

impl Point2D {
  pub fn translate(&mut self, vector: &Vector2D) {
    self.x += vector.x;
    self.y += vector.y;
  }
}


struct Vector2D {
  pub x: f32,
  pub y: f32,
}


pub struct Character {
  health: u32,
  location: Point2D,
  direction: Vector2D,
  actions: ActionSet,
  command_interpreter: CommandInterpreter,
}


impl Character {
  pub fn new() -> Self {
    Self {
      health: 0,
      location: Point2D{ x: 0.0, y: 0.0 },
      direction: Vector2D{ x: 0.0, y: 0.0 },
      actions: ActionSet::new(String::from("")),
      command_interpreter: CommandInterpreter::new(InputDevice(), Vec::new()),
    }
  }


  pub fn next(&mut self) {
    self.location.translate(&self.direction);
    let command: CommandPattern = self.command_interpreter.get_command();
    self.actions.next(command);
  }
}
