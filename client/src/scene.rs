use crate::gfx::programs::sprite::Sprite;
use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub struct Scene {
  pub sprites: Vec<Rc<Sprite>>
}