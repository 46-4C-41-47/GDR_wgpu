use wgpu::core::device;

struct Character {
  name: String
}


struct Stage {
  name: String
}


enum TimeLimit {
  None,
  Hundred = 99,
  Sixty = 60,
  Thirty = 30,
}


enum RoundType {
  One = 1,
  Three = 3,
  Five = 5
}


enum WinType {
  Regular,
  Special,
  OverDrive,
  Counter,
  Super,
  Ultra,
}


pub struct MatchDescriptior {
  left_team: Vec<Character>,
  right_team: Vec<Character>,
  stage: Stage,
  time_limit: TimeLimit,
  rounds: RoundType,
}


enum TeamType {
  Left,
  Right,
  None,
}


pub struct MatchResult {
  winner: TeamType,
  rounds: Vec<(TeamType, WinType)>,
  remaining_time: u8,
  left_team_score: u32,
  right_team_score: u32,
}


struct GdrEngine<'a> {
  surface: &'a wgpu::Surface<'a>,
  device: &'a wgpu::Device,
  queue: &'a wgpu::Queue,
}

impl<'a> GdrEngine<'a> {
  pub fn new(
    surface: &'a wgpu::Surface,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
  ) -> Self { 
    GdrEngine { surface, device, queue }
  }


  pub fn run_match(&self, match_to_create: MatchDescriptior) -> MatchResult {
    todo!();
  }
}
