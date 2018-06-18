#[derive(Serialize, Deserialize)]
pub struct GameResult {
    // id: Uuid,
    result: String,
    your_attack: String,
    their_attack: String
}

pub fn demo_draw_result() -> GameResult {
    GameResult{result: "draw".to_string(), your_attack: "spock".to_string(), their_attack: "spock".to_string()}
}

use actix::{Actor, Addr, Context, Handler, Message};

pub struct MyActor;

pub fn start() -> Addr<MyActor> {
    MyActor.start()
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
       println!("I am alive!");
    }
}
