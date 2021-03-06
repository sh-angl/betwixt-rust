use nalgebra::{Isometry3, Orthographic3, Perspective3, Point3, Vector2, Vector3};
use std::f32::consts::PI;

pub struct Camera {
  position: Vector2<f32>,
  screen_size: Vector2<f32>,
  projection: Orthographic3<f32>,
  zoom: f32,
}

impl Camera {
    pub fn new() -> Camera {

      Camera {
        position: Vector2::new(0.,0.),
        screen_size: Vector2::new(0.,0.),
        projection: Orthographic3::new(-1.0, 1.0, -1.0, 1.0, 0.1, 10.0),
        zoom: 1.,
      }
    }

    pub fn position(&self) -> Vector2<f32> {
      self.position
    }

    pub fn set_position(&mut self, new_pos: Vector2<f32>) {
      self.position = new_pos;
    }

    pub fn zoom(&self) -> f32 {
      self.zoom
    }

    pub fn set_zoom(&mut self, new_zoom: f32) {
      self.zoom = new_zoom;
      self.update_proj();
    }

    pub fn set_screen_size(&mut self, width: f64, height: f64) {
      self.screen_size.x = width as f32;
      self.screen_size.y = height as f32;
      self.update_proj();
    }

    fn update_proj(&mut self) {
      let mut zoom = self.zoom;
      if zoom < 0. {
        zoom = 1. / -zoom;
      }
      let width = self.screen_size.x;
      let height = self.screen_size.y;
      let scaled = 1. / zoom;
      if width > height {
        let aspect = (width / height) as f32 / zoom;
        self.projection.set_left_and_right(-aspect, aspect);
        self.projection.set_bottom_and_top(-scaled, scaled);
      } else {
        let aspect = (height / width) as f32 / zoom;
        self.projection.set_left_and_right(-scaled, scaled);
        self.projection.set_bottom_and_top(-aspect, aspect);
      }
    }

    pub fn view(&self) -> [f32; 16] {
        let view = Isometry3::translation(self.position.x, self.position.y, -5.0);

        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        view_array
    }
    pub fn projection(&self) -> [f32; 16] {
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(self.projection.as_matrix().as_slice());

        perspective_array
    }
}