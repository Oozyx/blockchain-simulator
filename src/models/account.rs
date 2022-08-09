pub struct AccountCollection {
  pub accounts: Vec<Account>,
}

impl AccountCollection {
  pub fn create_account(&mut self, id: u32, balance: u32) -> u32 {
    // increment num_accounts
    self.accounts.push(Account {
      id: id,
      balance: balance,
    });

    return id;
  }

  pub fn get_account(&mut self, id: u32) -> Result<&Account, ()> {
    let result = self.accounts.iter().find(|x| x.id == id);

    match result {
      Some(account) => {
        return Ok(account);
      }
      None => {
        return Err(());
      }
    }
  }

  pub fn account_exists(&self, id: u32) -> bool {
    let result = self.accounts.iter().find(|x| x.id == id);
    match result {
      Some(_) => {
        return true;
      }
      None => {
        return false;
      }
    }
  }

  pub fn transfer(&mut self, from: u32, to: u32, amount: u32) -> bool {
    if !self.account_exists(from) || !self.account_exists(to) {
      return false;
    }

    // check if from has enough
    let from_account = self.get_account(from);
    match from_account {
      Ok(account) => {
        if account.balance < amount {
          return false;
        }
      }
      Err(_) => {
        return false;
      }
    }

    self.decrease_balance(from, amount);
    self.increase_balance(to, amount);

    return true;
  }

  fn increase_balance(&mut self, id: u32, increase_amount: u32) {
    let result = self.accounts.iter_mut().find(|x| x.id == id);
    match result {
      Some(account) => {
        account.increase_balance(increase_amount);
      }
      None => {}
    }
  }

  fn decrease_balance(&mut self, id: u32, decrease_amount: u32) {
    let result = self.accounts.iter_mut().find(|x| x.id == id);
    match result {
      Some(account) => {
        account.decrease_balance(decrease_amount);
      }
      None => {}
    }
  }
}

pub struct Account {
  pub id: u32,
  pub balance: u32,
}

impl Account {
  pub fn increase_balance(&mut self, increase_amount: u32) {
    self.balance += increase_amount;
  }

  pub fn decrease_balance(&mut self, decrease_amount: u32) {
    self.balance -= decrease_amount;
  }
}
