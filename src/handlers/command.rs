use std::sync::Mutex;

use crate::config;
use crate::models;
use crate::routes;

pub async fn start_node_handler() {
  println!("Starting node...");
  let _rocket = rocket::build()
    .manage(Mutex::new(models::blockchain::Blockchain {
      accounts: models::account::AccountCollection {
        accounts: Vec::new(),
      },
    }))
    .mount("/", routes![routes::get::account_increment])
    .mount("/", routes![routes::get::account_create])
    .mount("/", routes![routes::get::account_transfer])
    .mount("/", routes![routes::get::account_balance])
    .launch()
    .await;
}

pub async fn create_account_handler(command: config::command::Command) {
  println!("Running create account command...");
  let mut command_str = String::from("http://127.0.0.1:8000/account/create/");
  command_str.push_str(command.args[0].as_str());
  command_str.push_str(String::from("/").as_str());
  command_str.push_str(command.args[1].as_str());
  let res = surf::get(command_str).await;
  match res {
    Ok(mut res_str) => {
      let res_body = res_str.body_string().await;
      match res_body {
        Ok(res_output) => {
          println!("{res_output}");
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

pub async fn transfer_handler(command: config::command::Command) {
  println!("Running transfer command...");
  let mut command_str = String::from("http://127.0.0.1:8000/account/transfer/");
  command_str.push_str(command.args[0].as_str());
  command_str.push_str(String::from("/").as_str());
  command_str.push_str(command.args[1].as_str());
  command_str.push_str(String::from("/").as_str());
  command_str.push_str(command.args[2].as_str());
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

pub async fn balance_handler(command: config::command::Command) {
  println!("Running balance command...");
  let mut command_str = String::from("http://127.0.0.1:8000/account/balance/");
  command_str.push_str(command.args[0].as_str());
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
