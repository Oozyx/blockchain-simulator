use std::env;

mod config;
mod handlers;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let config = config::parse::parse_program_args(env::args().collect());
    match config {
        Ok(res) => match res.name {
            config::command::Name::StartNode => {
                handlers::command::start_node_handler().await;
            }
            config::command::Name::CreateAccount => {
                handlers::command::create_account_handler(res).await;
            }
            config::command::Name::Transfer => {
                handlers::command::transfer_handler(res).await;
            }
            config::command::Name::Balance => {
                handlers::command::balance_handler(res).await;
            }
        },
        Err(reason) => {
            println!("{reason}");
        }
    }
}
