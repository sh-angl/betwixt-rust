use Player;
use serde:: {Serialize, Deserialize, Debug}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState{
    Players : [Player; 10]
}