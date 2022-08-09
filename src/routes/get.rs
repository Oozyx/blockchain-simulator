use rocket::State;
use std::sync::Mutex;

use crate::models;

#[get("/account/increment/<id>")]
pub fn account_increment(
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
pub fn account_create(
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
pub fn account_transfer(
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
pub fn account_balance(
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
