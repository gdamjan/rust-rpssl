use uuid::Uuid;
use actix_web::{App, Result, error, HttpRequest, HttpResponse};
use actix_web::fs::{NamedFile, StaticFiles};
use actix_web::http::{Method, header, StatusCode};

use rpssl;


// TODO
fn attack(_req: HttpRequest) -> Result<HttpResponse> {
    let result = rpssl::demo_draw_result();
    Ok(HttpResponse::Ok().json(result))
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
