use crate::gfx::programs::sprite::Sprite;
use std::{rc::Rc, sync::Arc};
use std::cell::{Ref, RefCell};

pub struct Scene {
  pub sprites: Vec<Arc<Sprite>>
}