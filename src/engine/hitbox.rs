#[derive(Copy, Clone, PartialEq)]
pub enum HitboxType {
  Attack,
  Guard,
  Grab,
  Body,
}


pub struct Square {
  pub center: (f32, f32),
  pub width: f32,
  pub height: f32,
}


pub struct Hitbox {
  square: Square,
  type_: HitboxType
}


impl Hitbox {
  pub fn new(square: Square, type_: HitboxType) -> Self {
    Self { square, type_ }
  }


  pub fn collide(&self, other: &Hitbox) -> bool {
    let x_spacing: f32 = self.square.center.0 - other.get_square().center.0;
    let y_spacing: f32 = self.square.center.1 - other.get_square().center.1;

    if x_spacing.abs() < self.square.width / 2.0 + other.get_square().width / 2.0 
    && y_spacing.abs() < self.square.height / 2.0 + other.get_square().height / 2.0 
    { true } else { false }
  }


  pub fn get_square(&self) -> &Square { &self.square }

  pub fn get_hitbox_type(&self) -> HitboxType { self.type_ }
}
