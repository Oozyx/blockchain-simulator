#[derive(Debug, PartialEq, Eq)]
pub enum Name {
  StartNode,
  CreateAccount,
  Transfer,
  Balance,
}

pub struct Command {
  pub name: Name,
  pub args: Vec<String>,
}
