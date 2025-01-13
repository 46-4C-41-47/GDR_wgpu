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


#[derive(Copy, Clone)]
pub enum Input {
  Neutral,
  
  Up,
  Down,
  Front,
  Back,

  UpFront,
  UpBack,
  DownFront,
  DownBack,

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

  drawables: Vec<Box<dyn Draw>>,
}

impl<'a> GdrEngine<'a> {
  pub fn new(
    surface: &'a wgpu::Surface,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
  ) -> Self { 
    GdrEngine { surface, device, queue, drawables: Vec::new() }
  }


  pub fn run_match(&self, match_to_create: MatchDescriptior) -> MatchResult {
    let mut current_match: Match = self.initialize_match(match_to_create);

    loop {
      let match_result: Option<MatchResult> = current_match.play();

      match match_result {
        Some(result) => return result,
        None => (),
      }

      let start: Instant = Instant::now();
      self.render().unwrap();
      let elapsed: Duration = Instant::now() - start;
      sleep(TIME_BETWEEN_FRAMES - elapsed);
    }
  }


  fn initialize_match(&self, match_to_create: MatchDescriptior) -> Match {
    todo!()
  }


  fn render(&self) -> Result<(), wgpu::SurfaceError> {
    let output: SurfaceTexture = self.surface.get_current_texture()?;
    let view: TextureView = output
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder: CommandEncoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
      });

    {
      let mut render_pass: RenderPass<'_> = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(DEFAULT_BG_COLOR),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None,
      });

      for drawable in &self.drawables {
        drawable.draw(&mut render_pass);
      }
    }

    self.queue.submit(iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}
