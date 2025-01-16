use std::collections::HashMap;
use std::vec;

use super::character::StateModification;
use super::gdr_engine::Input;
use super::geometry::Vector2D;
use super::hitbox::FrameHitbox;



#[derive(Eq, Hash, PartialEq)]
pub struct CommandPattern(pub Vec<Vec<Input>>);



pub enum CommonActions {
  Idle,
  //Fall,
  
  WalkFroward,
  WalkBackward,
  JumpForward,
  JumpBackward,
  Jump,
  
  Guard,
  //Parry,
  
  DashForward,
  DashBackward,
  
  LightPunch,
  MediumPunch,
  HeavyPunch,
  
  LightKick,
  MediumKick,
  HeavyKick,
}

impl CommonActions {
  pub fn get_command(&self) -> CommandPattern {
    match self {
              CommonActions::Idle => CommandPattern(vec![vec![Input::Neutral]]),
            //CommonActions::Fall => CommandPattern(vec![vec![Input::Neutral]]),
      
       CommonActions::WalkFroward => CommandPattern(vec![vec![Input::Forward]]),
      CommonActions::WalkBackward => CommandPattern(vec![vec![Input::Backward]]),
       CommonActions::JumpForward => CommandPattern(vec![vec![Input::UpForward]]),
      CommonActions::JumpBackward => CommandPattern(vec![vec![Input::UpBackward]]),
              CommonActions::Jump => CommandPattern(vec![vec![Input::Up]]),
      
             CommonActions::Guard => CommandPattern(vec![vec![Input::DownBackward]]),
           //CommonActions::Parry => CommandPattern(vec![vec![Input::Forward]]),
      
       CommonActions::DashForward => CommandPattern(vec![vec![Input::Forward], vec![Input::Forward]]),
      CommonActions::DashBackward => CommandPattern(vec![vec![Input::Backward], vec![Input::Backward]]),
      
        CommonActions::LightPunch => CommandPattern(vec![vec![Input::LightPunch]]),
       CommonActions::MediumPunch => CommandPattern(vec![vec![Input::MediumPunch]]),
        CommonActions::HeavyPunch => CommandPattern(vec![vec![Input::HeavyPunch]]),

         CommonActions::LightKick => CommandPattern(vec![vec![Input::LightKick]]),
        CommonActions::MediumKick => CommandPattern(vec![vec![Input::MediumKick]]),
         CommonActions::HeavyKick => CommandPattern(vec![vec![Input::HeavyKick]]),
    }
  }
}



pub struct ActionSet {
  actions: HashMap<CommandPattern, Action>,
  current: CommandPattern,
}

impl ActionSet {
  const DEFAULT_ACTION: CommonActions = CommonActions::Idle;

  
  pub fn new(json: String) -> Self { 
    Self {
      actions: HashMap::new(),
      current: Self::DEFAULT_ACTION.get_command(),
    }
  }


  pub fn next(&mut self, command: CommandPattern) -> StateModification {
    if !self.actions.get_mut(&self.current).unwrap().draw_next() {
      self.reset();
    }

    self.current = command;

    StateModification {
      health_substraction: 0,
      movement: Vector2D{ x: 0.0, y: 0.0 },
    }
  }


  pub fn reset(&mut self) { 
    self.current = Self::DEFAULT_ACTION.get_command();
    self.actions.get_mut(&self.current).unwrap().reset() 
  }


  pub fn set_action(&mut self, priority: bool, action: CommonActions) {
    self.current = action.get_command();
  }
}



struct ActionDescriptor {
  pub name: String,
  pub hitboxes: Vec<FrameHitbox>,
  pub command: CommandPattern,
}

struct Action {
  name: String,
  frame_count: u32,
  frame_index: u32,
  command: CommandPattern,
  hitboxes: Vec<FrameHitbox>,
  state_modification: StateModification,
}

impl Action {
  pub fn new(descriptor: ActionDescriptor) -> Self { 
    Self {
      name: descriptor.name,
      frame_count: descriptor.hitboxes.len() as u32,
      frame_index: 0,
      command: descriptor.command,
      hitboxes: descriptor.hitboxes,
      state_modification: StateModification{ health_substraction: 0, movement: Vector2D{ x: 0.0, y: 0.0 } },
    } 
  }


  pub fn reset(&mut self) { self.frame_index = 0 }


  pub fn draw_next(&mut self) -> bool {
    self.frame_index = (self.frame_index + 1) % self.frame_count;

    return self.frame_index != 0;
  }


  pub fn get_name(&self) -> &String { &self.name }

  pub fn get_hitbox(&self) -> &FrameHitbox { &self.hitboxes[self.frame_index as usize] }
  
  pub fn get_command(&self) -> &CommandPattern { &self.command }
}
