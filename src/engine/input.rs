use super::{
  action::Action, 
  gdr_engine::Input,
};


pub struct InputDevice(u32);


pub struct InputFlow {
  actions: Vec<Action>,
  input_device: InputDevice,
}


impl InputFlow {
  pub fn new(input_device: InputDevice, actions: Vec<Action>) -> Self { 
    Self { 
      input_device,
      actions,
    }
  }


  pub fn get_command(&self) -> Option<u32> { todo!() }
}
