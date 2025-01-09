use super::gdr_engine::{ 
  MatchResult,
  CharacterFile
};
use super::character::Character;
use super::hitbox::{Hitbox, HitboxType};


enum CollisionType {
  Hit,
  HitOnGuard,
  Grab,
}


pub struct Match {
  team_left: Vec<Character>,
  team_right: Vec<Character>,
}


impl Match {
  pub fn new(team_left: Vec<CharacterFile>, team_right: Vec<CharacterFile>) -> Self {
    Self { team_left: Vec::new(), team_right: Vec::new() }
  }


  pub fn play(&mut self) -> Option<MatchResult> {
    for left_character in &self.team_left {
      for right_character in &self.team_right {
        let a = Self::check_collisions(
          &left_character.get_hitbox(), 
          &right_character.get_hitbox()
        );
        let b = Self::check_collisions(
          &right_character.get_hitbox(), 
          &left_character.get_hitbox()
        );
        
      }
    }

    return None;
  }


  fn get_usable_hitbox<'a>(attackant: &'a Vec<Hitbox>, defender: &'a Vec<Hitbox>) 
  -> (HitboxType, Vec<&'a Hitbox>, Vec<&'a Hitbox>) {
    let offensive_hitboxs: Vec<&Hitbox> = attackant
      .iter()
      .filter(|hitbox: &&Hitbox| 
        hitbox.get_hitbox_type() == HitboxType::Attack || 
        hitbox.get_hitbox_type() == HitboxType::Grab)
      .collect();

    let offensive_hitbox_type: HitboxType = offensive_hitboxs[0].get_hitbox_type();
    for hitbox in &offensive_hitboxs {
      if hitbox.get_hitbox_type() != offensive_hitbox_type {
        panic!("A character can't do multiple offensive actions at the same time");
      }
    }

    let defensive_hitboxs: Vec<&Hitbox> = defender
      .iter()
      .filter(|hitbox: &&Hitbox| 
        hitbox.get_hitbox_type() == HitboxType::Body || 
        hitbox.get_hitbox_type() == HitboxType::Guard)
      .collect();

    (offensive_hitbox_type, offensive_hitboxs, defensive_hitboxs)
  }


  fn check_collisions(attackant: &Vec<Hitbox>, defender: &Vec<Hitbox>) -> Option<CollisionType> { 
    let (offensive_hitbox_type, 
      offensive_hitboxs, 
      defensive_hitboxs) = Self::get_usable_hitbox(attackant, defender);

    for offensive_hitbox in &offensive_hitboxs {
      for defensive_hitbox in &defensive_hitboxs {
        if offensive_hitbox.collide(defensive_hitbox) {
          if offensive_hitbox_type == HitboxType::Attack 
            && defensive_hitbox.get_hitbox_type() == HitboxType::Guard {
            return Some(CollisionType::HitOnGuard);
          }
          
          if offensive_hitbox_type == HitboxType::Attack 
            && defensive_hitbox.get_hitbox_type() == HitboxType::Body {
            return Some(CollisionType::Hit);
          }

          if offensive_hitbox_type == HitboxType::Grab 
            && (defensive_hitbox.get_hitbox_type() == HitboxType::Body || 
              defensive_hitbox.get_hitbox_type() == HitboxType::Guard) {
            return Some(CollisionType::Grab);
          }
        }
      }
    }

    return None;
  }
}
