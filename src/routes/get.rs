use rocket::State;
use std::sync::Mutex;
use std::thread;
use std::time;

use crate::models;

#[get("/account/increment/<id>")]
pub fn account_increment(
  id: u32,
  blockchain: &State<Mutex<models::blockchain::Blockchain>>,
) -> String {
  let mut lock = blockchain.lock().unwrap();

  // sleep until next block
  println!(
    "Seconds until block confirmation: {}",
    lock.get_time_until_next_block()
  );
  thread::sleep(time::Duration::from_secs(lock.get_time_until_next_block()));
  // find account
  let result = lock.accounts.accounts.iter_mut().find(|x| x.id == id);

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
  blockchain: &State<Mutex<models::blockchain::Blockchain>>,
) -> String {
  let mut lock = blockchain.lock().unwrap();

  // sleep until next block
  println!(
    "Seconds until block confirmation: {}",
    lock.get_time_until_next_block()
  );
  thread::sleep(time::Duration::from_secs(lock.get_time_until_next_block()));

  // create account
  let account_id = lock.accounts.create_account(id, balance);
  let mut res = String::from("Account id: ");
  res.push_str(account_id.to_string().as_str());

  return res;
}

#[get("/account/transfer/<from>/<to>/<amount>")]
pub fn account_transfer(
  from: u32,
  to: u32,
  amount: u32,
  blockchain: &State<Mutex<models::blockchain::Blockchain>>,
) -> String {
  let mut lock = blockchain.lock().unwrap();

  // sleep until next block
  println!(
    "Seconds until block confirmation: {}",
    lock.get_time_until_next_block()
  );
  thread::sleep(time::Duration::from_secs(lock.get_time_until_next_block()));

  if !lock.accounts.transfer(from, to, amount) {
    return String::from("Transfer failed.");
  } else {
    return String::from("Transfer success.");
  }
}

#[get("/account/balance/<id>")]
pub fn account_balance(
  id: u32,
  blockchain: &State<Mutex<models::blockchain::Blockchain>>,
) -> String {
  let mut lock = blockchain.lock().unwrap();

  let result = lock.accounts.get_account(id);
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
