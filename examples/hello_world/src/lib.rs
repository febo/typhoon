use crayfish_accounts::ProgramId;
use crayfish_handler_macro::handlers;
use pinocchio::{entrypoint, msg, program_error::ProgramError, pubkey::Pubkey};
use pinocchio_pubkey::declare_id;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub struct HelloWorldProgram;

impl ProgramId for HelloWorldProgram {
    const ID: Pubkey = crate::ID;
}

handlers! {
    hello_world,
}

pub fn hello_world() -> Result<(), ProgramError> {
    msg!("Hello World");
    Ok(())
}
