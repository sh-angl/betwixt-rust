use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player{
    pos: Vector<f64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector<T>{
    x: T,
    y: T
}