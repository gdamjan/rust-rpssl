#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Shape { Rock, Paper, Scissors, Spock, Lizard }

#[derive(Serialize, Clone)]
pub enum Outcome { Win, Draw, Loose }

#[derive(Serialize, Clone)]
pub struct GameResult {
    outcome: Outcome,
    your_attack: Shape,
    their_attack: Shape
}

fn play_rpssl(attack1: &Shape, attack2: &Shape) -> (GameResult, GameResult) {
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

    let res1 = GameResult{outcome: outcome1, your_attack: *attack1, their_attack: *attack2};
    let res2 = GameResult{outcome: outcome2, your_attack: *attack2, their_attack: *attack1};
    (res1, res2)
}

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use actix_web::actix::{Actor, Addr, Context, Handler, Message, ResponseFuture};

use futures::sync::oneshot::Sender;
use futures::sync::oneshot;
use futures::*;

pub struct GameActor {
    games: Arc<Mutex<HashMap<String, (Shape, Sender<GameResult>)>>>
}

pub fn start() -> Addr<GameActor> {
    let g = Arc::new(Mutex::new(HashMap::new()));
    GameActor{games: g}.start()
}

impl Actor for GameActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
       println!("Game Actor is alive!");
    }
}

pub struct Attack {
    pub game_id: String,
    pub attack: Shape,
}

impl Message for Attack {
    type Result = Result<GameResult, ()>;                 //  <─┐
}                                                         //    │
                                                          //    │  these two need to match, and it was not trivial to get them right
                                                          //    │            (the error messages didn't help much)
impl Handler<Attack> for GameActor {                      //    │
    type Result = ResponseFuture<GameResult, ()>;         //  <─┘

    fn handle(&mut self, msg: Attack, _: &mut Context<Self>) -> Self::Result {
        let mut games = self.games.lock().unwrap();
        let fut = if games.contains_key(&msg.game_id) {
            let (other_attack, tx) = games.remove(&msg.game_id).unwrap();
            let (res1, res2) = play_rpssl(&msg.attack, &other_attack);
            tx.send(res2);
            // I don't know how to make both of the `if` legs to be compatible, so:
            let (tx, rx) = oneshot::channel();
            tx.send(res1);
            rx
        } else {
            let (tx, rx) = oneshot::channel();
            games.insert(msg.game_id, (msg.attack, tx));
            rx
        };
        Box::new(fut.map(|x| x).map_err(|_| ()))
    }
}
