#[macro_use]
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate uuid;
extern crate tokio_timer;
extern crate futures;

mod webapp;
mod rpssl;

fn main() {
    let sys = actix::System::new("rpssl");
    let addr = "127.0.0.1:8088";

    actix_web::server::new(webapp::create_app)
        .bind(addr)
        .unwrap()
        .start();

    println!("Started http server: http://{}/", addr);
    let _ = sys.run();
}
