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

use super::{action::{self, ActionSet, CommandPattern}, geometry::{Point2D, Vector2D}, input::{CommandInterpreter, InputDevice}};


pub struct StateModification {
  pub health_substraction: u32,
  pub movement: Vector2D,
}


pub struct Character {
  health: u32,
  location: Point2D,
  direction: Vector2D,
  is_airborne: bool,
  actions: ActionSet,
  command_interpreter: CommandInterpreter,
}


impl Character {
  const GRAVITY: Vector2D = Vector2D{ x: 0.0, y: -0.1 };


  pub fn new() -> Self {
    Self {
      health: 0,
      location: Point2D{ x: 0.0, y: 0.0 },
      direction: Vector2D{ x: 0.0, y: 0.0 },
      is_airborne: false,
      actions: ActionSet::new(String::from("")),
      command_interpreter: CommandInterpreter::new(InputDevice(), Vec::new()),
    }
  }


  pub fn next(&mut self) {
    self.run_physics();
    let command: CommandPattern = self.command_interpreter.get_command();

    if !self.is_airborne {
      let modification: StateModification = self.actions.next(command);
      self.apply_state_modification(modification);
    }
  }


  fn apply_state_modification(&mut self, state_modification: StateModification) {
    self.health -= state_modification.health_substraction;
    self.direction = state_modification.movement;
  }


  fn run_physics(&mut self) {
    self.location.translate(&self.direction);

    if self.is_airborne {
      self.direction.apply(&Self::GRAVITY);
    } else {
      self.direction = Vector2D{ x: 0.0, y: 0.0 };
    }

    if self.location.y < 0.0 {
      self.location.y = 0.0;
      self.is_airborne = false;
    }
  }
}
