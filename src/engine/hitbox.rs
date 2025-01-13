#[derive(Copy, Clone, PartialEq)]
pub enum HitboxType {
  Attack,
  Hurt,
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


pub struct FrameHitbox {
  pub hitboxes: Vec<Hitbox>,
}


impl FrameHitbox {
  pub fn new() -> Self {
    Self { hitboxes: Vec::new() }
  }


  pub fn get_attack_hitbox(&self) -> Option<Vec<Hitbox>> { todo!() }

  pub fn get_grab_hitbox(&self) -> Option<Vec<Hitbox>> { todo!() }

  pub fn get_hurt_hitbox(&self) -> Option<Vec<Hitbox>> { todo!() }

  pub fn get_guard_hitbox(&self) -> Option<Vec<Hitbox>> { todo!() }

  pub fn get_body_hitbox(&self) -> Option<Vec<Hitbox>> { todo!() }
}
