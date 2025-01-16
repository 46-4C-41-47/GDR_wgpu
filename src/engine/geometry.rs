#[derive(Copy, Clone)]
pub struct Point2D {
  pub x: f32,
  pub y: f32,
}

impl Point2D {
  pub fn translate(&mut self, vector: &Vector2D) {
    self.x += vector.x;
    self.y += vector.y;
  }
}



#[derive(Copy, Clone)]
pub struct Vector2D {
  pub x: f32,
  pub y: f32,
}

impl Vector2D {
  pub fn apply(&mut self, vector: &Vector2D) {
    self.x += vector.x;
    self.y += vector.y;
  }
}



#[derive(Copy, Clone)]
pub struct USize {
  pub width: u32,
  pub height: u32
}



#[derive(Copy, Clone)]
pub struct FSize {
  pub width: f32,
  pub height: f32
}
