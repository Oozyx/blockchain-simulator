use rocket::State;
use std::env;
use std::sync::Mutex;

mod config;
mod models;

#[macro_use]
extern crate rocket;

#[get("/account/increment/<id>")]
fn account_increment(
    id: u32,
    account_collection: &State<Mutex<models::account::AccountCollection>>,
) -> String {
    let mut lock = account_collection.lock().unwrap();
    // find account
    let result = lock.accounts.iter_mut().find(|x| x.id == id);

    match result {
        Some(account) => {
            account.increase_balance(1);
            let new_balance = account.balance.to_string();
            let mut res = String::from("New balance: ");
            res.push_str(new_balance.as_str());

            return res;
        }
        None => {
            return String::from("Invalid Id");
        }
    }
}

#[get("/account/create/<id>/<balance>")]
fn account_create(
    id: u32,
    balance: u32,
    account_collection: &State<Mutex<models::account::AccountCollection>>,
) -> String {
    let mut lock = account_collection.lock().unwrap();
    // create account
    let account_id = lock.create_account(id, balance);
    let mut res = String::from("Account id: ");
    res.push_str(account_id.to_string().as_str());

    return res;
}

#[get("/account/transfer/<from>/<to>/<amount>")]
fn account_transfer(
    from: u32,
    to: u32,
    amount: u32,
    account_collection: &State<Mutex<models::account::AccountCollection>>,
) -> String {
    let mut lock = account_collection.lock().unwrap();

    if !lock.transfer(from, to, amount) {
        return String::from("Transfer failed.");
    } else {
        return String::from("Transfer success.");
    }
}

#[get("/account/balance/<id>")]
fn account_balance(
    id: u32,
    account_collection: &State<Mutex<models::account::AccountCollection>>,
) -> String {
    let mut lock = account_collection.lock().unwrap();

    let result = lock.get_account(id);
    match result {
        Ok(account) => {
            let mut res = String::from("Account id: ");
            res.push_str(account.balance.to_string().as_str());
            return res;
        }
        Err(_) => {
            return String::from("Could not get account balance.");
        }
    }
}

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
                    .mount("/", routes![account_increment])
                    .mount("/", routes![account_create])
                    .mount("/", routes![account_transfer])
                    .mount("/", routes![account_balance])
                    .launch()
                    .await?;
            }
            config::command::Name::CreateAccount => {
                println!("Running create account command...");
                let mut command_str = String::from("http://127.0.0.1:8000/account/create/");
                command_str.push_str(res.args[0].as_str());
                command_str.push_str(String::from("/").as_str());
                command_str.push_str(res.args[1].as_str());
                let res = surf::get(command_str).await;
                match res {
                    Ok(mut res_str) => {
                        let res_str_str = res_str.body_string().await;
                        match res_str_str {
                            Ok(temp) => {
                                println!("{temp}");
                            }
                            Err(_) => {
                                println!("Error");
                            }
                        }
                    }
                    Err(_) => {
                        println!("Error");
                    }
                }
            }
            config::command::Name::Transfer => {
                println!("Running transfer command...");
                let mut command_str = String::from("http://127.0.0.1:8000/account/transfer/");
                command_str.push_str(res.args[0].as_str());
                command_str.push_str(String::from("/").as_str());
                command_str.push_str(res.args[1].as_str());
                command_str.push_str(String::from("/").as_str());
                command_str.push_str(res.args[2].as_str());
                let res = surf::get(command_str).await;
                match res {
                    Ok(mut res_str) => {
                        let res_str_str = res_str.body_string().await;
                        match res_str_str {
                            Ok(temp) => {
                                println!("{temp}");
                            }
                            Err(_) => {
                                println!("Error");
                            }
                        }
                    }
                    Err(_) => {
                        println!("Error");
                    }
                }
            }
            config::command::Name::Balance => {
                println!("Running balance command...");
                let mut command_str = String::from("http://127.0.0.1:8000/account/balance/");
                command_str.push_str(res.args[0].as_str());
                let res = surf::get(command_str).await;
                match res {
                    Ok(mut res_str) => {
                        let res_str_str = res_str.body_string().await;
                        match res_str_str {
                            Ok(temp) => {
                                println!("{temp}");
                            }
                            Err(_) => {
                                println!("Error");
                            }
                        }
                    }
                    Err(_) => {
                        println!("Error");
                    }
                }
            }
        },
        Err(reason) => {
            println!("{reason}");
        }
    }

    Ok(())
}
