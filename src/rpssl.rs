#[derive(Serialize, Deserialize)]
pub struct GameResult {
    result: String,
    your_attack: String,
    their_attack: String
}

pub fn demo_draw_result() -> GameResult {
    GameResult{result: "draw".to_string(), your_attack: "spock".to_string(), their_attack: "spock".to_string()}
}
