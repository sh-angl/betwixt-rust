use crate::camera::Camera;
use common::game_state::GameState;

pub struct State {
  pub time: f64,
  pub dt: f64,
  pub camera: Camera,
  pub dirty_screen: bool,
  pub game_state: GameState,
}