use super::{action::CommandPattern, gdr_engine::Input};


pub struct InputDevice();


pub struct CommandInterpreter {
  input_device: InputDevice,
  actions: Vec<CommandPattern>,
}


impl CommandInterpreter {
  pub fn new(input_device: InputDevice, actions: Vec<CommandPattern>) -> Self { 
    Self { 
      input_device,
      actions,
    }
  }


  pub fn get_command(&self) -> CommandPattern { CommandPattern(vec![vec![Input::Neutral]]) }
}
