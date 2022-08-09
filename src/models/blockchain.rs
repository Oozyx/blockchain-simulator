use std::time;

use crate::models;

pub struct Blockchain {
  pub accounts: models::account::AccountCollection,
}

impl Blockchain {
  pub fn get_time_until_next_block(&self) -> u64 {
    // get current timestamp in seconds
    let start = time::SystemTime::now();
    let since_the_epoch = start
      .duration_since(time::UNIX_EPOCH)
      .expect("Time went backwards");
    let remainder = since_the_epoch.as_secs() % 10;

    return 10 - remainder;
  }
}
