use crate::camera::Camera;

pub struct State {
  pub time: f64,
  pub dt: f64,
  pub camera: Camera,
  pub dirty_screen: bool,
}