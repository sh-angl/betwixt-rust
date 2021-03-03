use nalgebra::{Isometry3, Orthographic3, Perspective3, Point3, Vector2, Vector3};
use std::f32::consts::PI;

pub struct Camera {
  position: Vector2<f32>,
  projection: Orthographic3<f32>,
}

impl Camera {
    pub fn new() -> Camera {

      Camera {
        position: Vector2::new(0.,0.),
        projection: Orthographic3::new(-1.0, 1.0, -1.0, 1.0, 0.1, 10.0),
      }
    }

    pub fn position(&self) -> Vector2<f32> {
      self.position
    }

    pub fn set_position(&mut self, new_pos: Vector2<f32>) {
      self.position = new_pos;
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