use std::time::{Duration, Instant};
use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use futures::Future;
use tokio_timer;

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

pub fn demo_draw_result(attack: Shape) -> GameResult {
    let other = attack.clone();
    GameResult{outcome: Outcome::Draw, your_attack: attack,  their_attack: other}
}

pub struct GameActor;

pub fn start() -> Addr<GameActor> {
    GameActor.start()
}

impl Actor for GameActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
       println!("Game Actor is alive!");
    }
}

impl Message for Shape {
    type Result = Result<GameResult, ()>;
}

impl Handler<Shape> for GameActor {
    type Result = ResponseFuture<GameResult, ()>;

    fn handle(&mut self, msg: Shape, _: &mut Context<Self>) -> Self::Result {
        let game_outcome : GameResult = demo_draw_result(msg);

        let when = Instant::now() + Duration::new(3, 0);
        let fut = tokio_timer::Delay::new(when)
           .map(|_| game_outcome)
           .map_err(|_| ());
        Box::new(fut)
    }
}
