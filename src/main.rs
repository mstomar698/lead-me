#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::Redirect;
pub mod utils;

#[get("/")]
fn index() -> &'static str {
    "hello #Mayank"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("This is page with {} as cmd", cmd);
    let command = utils::get_command_from_q_s(&cmd);
    let redirect_url = match command {
        "gh" => utils::github::construct_gh_url(&cmd),
        "tw" => utils::twitter::construct_tw_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };
    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite().mount("/", routes![index, search]).launch();
}
