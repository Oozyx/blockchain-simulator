# Blockchain Simulator

## Installation

Need to install rust and cargo

## How to Use

In terminal 1 run: `cargo run start-node`

This will launch the blockchain and 1 block will get mined every 10 seconds.

The balance function does not need to wait until a block gets mined.

In terminal 2 run:

`cargo run create-account <id> <inital_balance>`

`cargo run transfer <id_from> <id_to> <amount>`

`cargo run balance <id>`


Where id is a number (i.e. 1, 2, 3, 4...) 
  and
Where amount, initial_balance are also numbers
