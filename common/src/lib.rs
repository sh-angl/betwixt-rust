use serde::{Serialize, Deserialize};

pub mod game_state;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player{
    pos: Vector<f64>,
    vel: Vector<f64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector<T>{
    x: T,
    y: T
}