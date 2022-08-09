use std::env;
use std::sync::Mutex;

mod config;
mod handlers;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = config::parse::parse_program_args(env::args().collect());
    match config {
        Ok(res) => match res.name {
            config::command::Name::StartNode => {
                println!("Starting node...");
                let _rocket = rocket::build()
                    .manage(Mutex::new(models::account::AccountCollection {
                        accounts: Vec::new(),
                    }))
                    .mount("/", routes![routes::get::account_increment])
                    .mount("/", routes![routes::get::account_create])
                    .mount("/", routes![routes::get::account_transfer])
                    .mount("/", routes![routes::get::account_balance])
                    .launch()
                    .await?;
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

    Ok(())
}
