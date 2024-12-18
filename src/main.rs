use my_window::MyWindow;
use winit::event_loop::EventLoop;

mod my_window;
mod window_state;
mod vertex;
mod params;
mod texture;


// manette -> gilrs
// IHM -> egui
fn main() {
  let my_event_loop: EventLoop<()> = EventLoop::new().unwrap();
  
  let mut app: MyWindow = my_window::MyWindow::new(String::from("Title"), (600, 400), false);

  my_event_loop.run_app(&mut app).unwrap();
}
