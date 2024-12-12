use wgpu::rwh::{HasRawWindowHandle, HasWindowHandle, RawWindowHandle};
use winit::{
  application::ApplicationHandler, 
  dpi::{ PhysicalSize, Size }, 
  event::{self, WindowEvent},
  keyboard::{KeyCode, PhysicalKey}
};
use crate::window_state::WindowSate;


pub struct MyWindow<'a> {
  title: String,
  size: winit::dpi::PhysicalSize<u32>,
  full_screen: bool,

  window: Option<winit::window::Window>,
  window_state: Option<WindowSate<'a>>,
}


impl<'a> MyWindow<'a> {
  pub fn new(title: String, size: (u32, u32), full_screen:bool) -> Self {
    Self {
      title,
      size: PhysicalSize::new(size.0, size.1),
      full_screen,
      window: None,
      window_state: None
    }
  }


  pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
    
    Ok(())
  }
}


impl<'a> ApplicationHandler for MyWindow<'a> {
  fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
    let attrib = winit::window::Window::default_attributes()
      .with_title(self.title.clone())
      .with_inner_size(self.size.clone());

    self.window = Some(event_loop.create_window(attrib).unwrap());
    self.window_state = Some(WindowSate::new(self.window.as_ref().unwrap()));

    if self.full_screen {
      self.window.as_ref().unwrap().set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    }

    //let window_state = WindowSate::new(self.window.as_ref().unwrap());
  }
  

  fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: event::WindowEvent,
  ) {
    if window_id != self.window.as_ref().unwrap().id() {
      return;
    }

    match event {
      event::WindowEvent::CloseRequested | event::WindowEvent::KeyboardInput { 
        event: event::KeyEvent { 
          state: event::ElementState::Pressed, 
          physical_key: PhysicalKey::Code(KeyCode::Escape), 
          .. 
        }, 
        .. 
      } => {
        println!("exiting window");
        event_loop.exit();
      },

      WindowEvent::RedrawRequested => {
        self.window.as_ref().unwrap().request_redraw();
        //self.render();
      },

      _ => (),  
    }
  }
}

