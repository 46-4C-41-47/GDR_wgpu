use std::sync::Arc;
use winit::{
  application::ApplicationHandler, 
  dpi::PhysicalSize, 
  event::{self, WindowEvent},
  keyboard::{KeyCode, PhysicalKey}
};
use crate::window_state::{self, WindowSate};


pub struct MyWindow {
  title: String,
  size: winit::dpi::PhysicalSize<u32>,
  full_screen: bool,

  window: Option<Arc<winit::window::Window>>,
  window_state: Option<window_state::WindowSate>,
}


impl MyWindow {
  const BACKGROUND_COLOR: wgpu::Color = wgpu::Color { r: 0.1, g: 0.1, b: 0.2, a: 1.0 };


  pub fn new(title: String, size: (u32, u32), full_screen:bool) -> Self {
    Self {
      title,
      size: PhysicalSize::new(size.0, size.1),
      full_screen,
      window: None,
      window_state: None
    }
  }


  fn toggle_fullscreen(&mut self) {
    if self.full_screen {
      self.window.as_ref().unwrap().set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    } else {
      self.window.as_ref().unwrap().set_fullscreen(None);
    }
  }


  fn handle_redraw(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    self.window.as_ref().unwrap().request_redraw();
        
    match self.window_state.as_ref().unwrap().render(Self::BACKGROUND_COLOR) {
      Ok(_) => {},

      Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) 
        => self.window_state.as_mut().unwrap().resize(self.size),
        
      Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
      
      Err(wgpu::SurfaceError::OutOfMemory) => {
        log::error!("OutOfMemory");
        event_loop.exit();
      }
    }
  }
}


impl ApplicationHandler for MyWindow {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    if self.window.is_some() { return; }

    let attrib = winit::window::Window::default_attributes()
      .with_title(self.title.clone())
      .with_inner_size(self.size.clone());

    let window: Arc<winit::window::Window> = Arc::new(
      event_loop.create_window(attrib).unwrap()
    );

    self.window = Some(window.clone());
    self.window_state = Some(WindowSate::new(window));

    self.toggle_fullscreen();
  }
  

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: event::WindowEvent,
  ) {
    if window_id != self.window.as_ref().unwrap().id() { return; }

    if self.window_state.as_mut().unwrap().process_input(&event) { return; }

    match event {
      event::WindowEvent::CloseRequested | event::WindowEvent::KeyboardInput { 
        event: event::KeyEvent { 
          state: event::ElementState::Pressed, 
          physical_key: PhysicalKey::Code(KeyCode::Escape), 
          .. 
        }, 
        .. 
      } => event_loop.exit(),

      event::WindowEvent::KeyboardInput { 
        event: event::KeyEvent { 
          state: event::ElementState::Pressed, 
          physical_key: PhysicalKey::Code(KeyCode::F1), 
          .. 
        }, 
        .. 
      } => {
        self.full_screen = !self.full_screen;
        self.toggle_fullscreen();
      },

      WindowEvent::Resized(physical_size) => {
        log::info!("physical_size: {physical_size:?}");
        self.window_state.as_mut().unwrap().resize(physical_size);
      }

      WindowEvent::RedrawRequested => self.handle_redraw(event_loop),

      _ => (),  
    }
  }
}

