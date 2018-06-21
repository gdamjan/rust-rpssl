#[derive(Serialize, Clone)]
pub enum Shape { Rock, Paper, Scissors, Spock, Lizard }

#[derive(Serialize)]
pub enum Outcome { Win, Draw, Loose }

#[derive(Serialize)]
pub struct GameResult {
    // id: Uuid,
    outcome: Outcome,
    your_attack: Shape,
    their_attack: Shape
}

pub fn demo_draw_result(shape: Shape) -> GameResult {
    GameResult{outcome: Outcome::Draw, your_attack: shape.clone(),  their_attack: shape}
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
