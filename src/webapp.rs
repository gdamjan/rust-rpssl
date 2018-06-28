use uuid::Uuid;
use actix;
use actix_web::{App, Result, error, HttpRequest, HttpResponse, Path, Json};
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::{Method, header, StatusCode};
use futures::Future;

use rpssl;

type AppState = actix::Addr<rpssl::GameActor>;

#[derive(Deserialize)]
struct AttackJson { attack: rpssl::Shape }

fn attack(data: (HttpRequest<AppState>, Path<String>, Json<AttackJson>)) -> Box<Future<Item=HttpResponse, Error=error::Error>> {
    let (req, path, val) = data;
    let msg = rpssl::Attack{game_id: path.to_string(), attack: val.attack};
    let actor = req.state();
    let fut = actor.send(msg)
        .map(|response| HttpResponse::build(StatusCode::OK).json(response.unwrap()))
        .map_err(|_| error::ErrorBadRequest("some error"));

    Box::new(fut)
}

fn newgame(req: HttpRequest<AppState>) -> Result<HttpResponse> {
    let game_id = Uuid::new_v4().hyphenated().to_string();
    let url = req.url_for("gamepage", &[game_id])?;
    Ok(HttpResponse::Found()
       .header(header::LOCATION, url.as_str())
       .finish())
}

fn mainpage(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/main.html")?)
}
fn gamepage(_req: HttpRequest<AppState>) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/game.html")?)
}

pub fn create_app(state: AppState) -> App<AppState> {
    App::with_state(state)
        .handler("/static", StaticFiles::new("static"))
        .resource("/{id}/attack", |r| r.method(Method::POST).with(attack))
        .resource("/{id}/", |r| { r.name("gamepage"); r.method(Method::GET).f(gamepage) })
        .route("/", Method::POST, newgame)
        .route("/", Method::GET, mainpage)
}
