use uuid::Uuid;
use actix;
use actix_web::{App, Result, error, HttpRequest, HttpResponse};
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::{Method, header, StatusCode};
use futures::Future;

use rpssl;

type AppState = actix::Addr<rpssl::GameActor>;


// TODO
fn attack(req: HttpRequest<AppState>) -> Box<Future<Item=HttpResponse, Error=error::Error>> {
    let this_is_fake = rpssl::Attack{attack:rpssl::Shape::Spock};
    let fut = req.state().send(this_is_fake)
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
        .resource("/{id}/attack", |r| r.method(Method::POST).f(attack))
        .resource("/{id}/", |r| { r.name("gamepage"); r.method(Method::GET).f(gamepage) })
        .route("/", Method::POST, newgame)
        .route("/", Method::GET, mainpage)
}
