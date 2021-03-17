use crate::Player;
use serde:: {Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState{
    pub players : Vec<Player>
}