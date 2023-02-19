#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
use bin::websys_worker_handler::HandleWorkerFtl;
use dwy_vest::{bin, App};
use rocket::fs::FileServer;
use yew_agent::PublicWorker;

#[get("/")]
fn index() -> &'static str {
    "/^(index)?[\\w]{5,}$/"
}

#[get("/about")]
fn about() -> &'static str {
    "/^(about)?[\\w]{5,}$/"
}

#[get("/account")]
fn account() -> &'static str {
    "/^(account)?[\\w]{7,}$/"
}

#[get("/login")]
fn login() -> &'static str {
    "/^(login)?[\\w]{5,}$/"
}

#[get("/localize")]
fn localize() -> &'static str {
    "/^(localize)?[\\w]{8,}$/"
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
//         .mount("/", routes![about])
//         .mount("/", routes![login])
//         .mount("/", routes![account])
//         .mount("/", routes![localize]);
// }

#[launch]
fn rocket() -> _ {
    HandleWorkerFtl::register();
    yew::Renderer::<App>::new().render();

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![about])
        .mount("/", routes![login])
        .mount("/", routes![account])
        .mount("/", routes![localize])
        .mount("/public", FileServer::from("static/"))
}
