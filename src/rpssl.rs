use std::time::{Duration, Instant};
use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};
use futures::Future;
use tokio_timer;

#[derive(Serialize, Clone, Copy, PartialEq)]
pub enum Shape { Rock, Paper, Scissors, Spock, Lizard }

#[derive(Serialize)]
pub enum Outcome { Win, Draw, Loose }

#[derive(Serialize)]
pub struct GameResult {
    outcome: Outcome,
    your_attack: Shape,
    their_attack: Shape
}

fn play_rpssl(attack1: Shape, attack2: Shape) -> (GameResult, GameResult) {
    let (outcome1, outcome2) = match (attack1, attack2) {
        (Shape::Rock, Shape::Lizard) => (Outcome::Win, Outcome::Loose),
        (Shape::Paper, Shape::Rock) => (Outcome::Win, Outcome::Loose),
        (Shape::Scissors, Shape::Paper) => (Outcome::Win, Outcome::Loose),
        (Shape::Spock, Shape::Scissors) => (Outcome::Win, Outcome::Loose),
        (Shape::Lizard, Shape::Spock) => (Outcome::Win, Outcome::Loose),
        (Shape::Rock, Shape::Scissors) => (Outcome::Win, Outcome::Loose),
        (Shape::Paper, Shape::Spock) => (Outcome::Win, Outcome::Loose),
        (Shape::Scissors, Shape::Lizard) => (Outcome::Win, Outcome::Loose),
        (Shape::Spock, Shape::Rock) => (Outcome::Win, Outcome::Loose),
        (Shape::Lizard, Shape::Paper) => (Outcome::Win, Outcome::Loose),
        (_, _) if attack1 == attack2 => (Outcome::Draw, Outcome::Draw),
        (_, _) =>  (Outcome::Loose, Outcome::Win),
    };

    let res1 = GameResult{outcome: outcome1, your_attack: attack1, their_attack: attack2};
    let res2 = GameResult{outcome: outcome2, your_attack: attack2, their_attack: attack1};
    (res1, res2)
}

pub fn demo_draw_result(attack: Shape) -> GameResult {
    let other = attack.clone();
    play_rpssl(attack, other).0
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

pub struct Attack {
    pub attack: Shape,
}

impl Message for Attack {
    type Result = Result<GameResult, ()>;
}

impl Handler<Attack> for GameActor {
    type Result = ResponseFuture<GameResult, ()>;

    fn handle(&mut self, msg: Attack, _: &mut Context<Self>) -> Self::Result {
        let game_outcome : GameResult = demo_draw_result(msg.attack);

        let when = Instant::now() + Duration::new(3, 0);
        let fut = tokio_timer::Delay::new(when)
           .map(|_| game_outcome)
           .map_err(|_| ());
        Box::new(fut)
    }
}
