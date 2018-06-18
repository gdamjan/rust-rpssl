use std::time::{Duration, Instant};

use uuid::Uuid;
use actix_web::{App, Result, error, HttpRequest, HttpResponse};
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::{Method, header, StatusCode};
use tokio_timer;
use futures::Future;

use rpssl;


// TODO
fn attack(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=error::InternalError<tokio_timer::Error>>> {
    let when = Instant::now() + Duration::new(3, 0);
    let future = tokio_timer::Delay::new(when)
        .and_then(|_| Ok(rpssl::demo_draw_result()))
        .and_then(|result| Ok(HttpResponse::build(StatusCode::OK).json(result)))
        .map_err(|e| error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR));

    Box::new(future)
}

fn newgame(req: HttpRequest) -> Result<HttpResponse> {
    let game_id = Uuid::new_v4().hyphenated().to_string();
    let url = req.url_for("gamepage", &[game_id])?;
    Ok(HttpResponse::Found()
       .header(header::LOCATION, url.as_str())
       .finish())
}

fn mainpage(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/main.html")?)
}
fn gamepage(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/game.html")?)
}

pub fn create_app() -> App {
    App::new()
        .handler("/static", StaticFiles::new("static"))
        .resource("/{id}/attack", |r| r.method(Method::POST).f(attack))
        .resource("/{id}/", |r| { r.name("gamepage"); r.method(Method::GET).f(gamepage) })
        .route("/", Method::POST, newgame)
        .route("/", Method::GET, mainpage)
}
