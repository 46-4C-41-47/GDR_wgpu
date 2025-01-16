use std::{
  iter, 
  thread::sleep, 
  time::{ Duration, Instant }
};

use wgpu::{ 
  CommandEncoder, 
  RenderPass, 
  SurfaceTexture, 
  TextureView 
};

use crate::engine::my_match::Match;

pub struct CharacterFile(String);


pub struct Stage(String);


#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Input {
  Neutral,
  
  Up,
  Down,
  Forward,
  Backward,

  UpForward,
  UpBackward,
  DownForward,
  DownBackward,

  QuarterForward,
  QuarterBackward,
  HalfForward,
  HalfBackward,
  ZForward,
  ZBackward,
  CircleForward,
  CircleBackward,

  LightPunch,
  MediumPunch,
  HeavyPunch,

  LightKick,
  MediumKick, 
  HeavyKick,
}


pub enum TimeLimit {
  None,
  Hundred = 99,
  Sixty = 60,
  Thirty = 30,
}


pub enum RoundType {
  One = 1,
  Three = 3,
  Five = 5
}


pub enum WinType {
  Regular,
  Special,
  OverDrive,
  Counter,
  Super,
  Ultra,
}


pub struct MatchDescriptior {
  left_team: Vec<CharacterFile>,
  right_team: Vec<CharacterFile>,
  stage: Stage,
  time_limit: TimeLimit,
  rounds: RoundType,
}


pub enum TeamType {
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


trait Draw {
  fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>);
}


static DEFAULT_BG_COLOR: wgpu::Color = wgpu::Color::BLACK;
static FRAMERATE: u8 = 60;
static TIME_BETWEEN_FRAMES: Duration = Duration::from_millis(1000 / FRAMERATE as u64);

pub struct GdrEngine<'a> {
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
    let mut current_match: Match = self.initialize_match(match_to_create);

    loop {
      let start: Instant = Instant::now();
      let match_result: Option<MatchResult> = current_match.play();

      match match_result {
        Some(result) => return result,
        None => (),
      }

      let elapsed: Duration = Instant::now() - start;
      sleep(TIME_BETWEEN_FRAMES - elapsed);
    }
  }


  fn initialize_match(&self, match_to_create: MatchDescriptior) -> Match {
    todo!()
  }
}
