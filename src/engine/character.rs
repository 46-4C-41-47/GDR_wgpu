use super::hitbox::Hitbox;


pub struct Character {
  pub id: u32,
  pub health: u32,
  pub super_: u32,
  pub stun: u32,
}


impl Character {
  pub fn new(id: u32) -> Self {
    Self {
      id,
      health: 0,
      super_: 0,
      stun: 0,
    }
  }


  pub fn get_hitbox(&self) -> Vec<Hitbox> { todo!() }
}
