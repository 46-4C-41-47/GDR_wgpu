use crate::gdr_engine::MatchResult;


struct CharacterStatus {
  health: u32,
  super_level: u32,
  stun: u32,
  score: u32,
}


pub struct Match {
  team_left: Vec<CharacterStatus>,
  team_right: Vec<CharacterStatus>,
}


impl Match {
  pub fn new() -> Self {
    Self { team_left: Vec::new(), team_right: Vec::new() }
  }


  pub fn play(&mut self) -> Option<MatchResult> {
    return None;
  }
}
