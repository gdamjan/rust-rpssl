#[macro_use]
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate uuid;
extern crate futures;

use std::env;

mod webapp;
mod rpssl;

fn main() {
    let sys = actix::System::new("rpssl");
    let bind_addr = env::var("HTTP_ADDR").unwrap_or("127.0.0.1:8088".to_string());

    let actor_addr = rpssl::start();

    actix_web::server::new(move || webapp::create_app(actor_addr.clone()))
        .bind(&bind_addr)
        .unwrap()
        .start();

    println!("Started http server: http://{}/", &bind_addr);
    let _ = sys.run();
}
