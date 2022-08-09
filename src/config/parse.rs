use super::command;

pub fn parse_program_args(args: Vec<String>) -> Result<command::Command, &'static str> {
  if args.len() == 1 {
    return Err("No arguments given.");
  }
  match args[1].to_lowercase().as_str() {
    "start-node" => {
      return parse_start_node(args);
    }
    "create-account" => {
      return parse_create_account(args);
    }
    "transfer" => {
      return parse_transfer(args);
    }
    "balance" => {
      return parse_balance(args);
    }
    _ => {
      return Err("Invalid program argument");
    }
  }
}

fn parse_start_node(args: Vec<String>) -> Result<command::Command, &'static str> {
  if args.len() != 2 {
    return Err("Invalid start-node arguments.");
  } else {
    return Ok(command::Command {
      name: command::Name::StartNode,
      args: Vec::new(),
    });
  }
}

fn parse_create_account(args: Vec<String>) -> Result<command::Command, &'static str> {
  if args.len() != 4 {
    return Err("Invalid create-account arguments.");
  } else {
    return Ok(command::Command {
      name: command::Name::CreateAccount,
      args: args[2..4].to_vec(),
    });
  }
}

fn parse_transfer(args: Vec<String>) -> Result<command::Command, &'static str> {
  if args.len() != 5 {
    return Err("Invalid transfer arguments.");
  } else {
    return Ok(command::Command {
      name: command::Name::Transfer,
      args: args[2..5].to_vec(),
    });
  }
}

fn parse_balance(args: Vec<String>) -> Result<command::Command, &'static str> {
  if args.len() != 3 {
    return Err("Invalid balance arguments.");
  } else {
    return Ok(command::Command {
      name: command::Name::Balance,
      args: args[2..3].to_vec(),
    });
  }
}
