use crayfish_accounts::{Owner, Program, System};
use crayfish_context_macro::context;
use crayfish_handler_macro::handlers;
use crayfish_program_id_macro::program_id;
use pinocchio::{entrypoint, msg, program_error::ProgramError};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[context]
pub struct InitContext<'a> {
    pub system: Program<'a, System>,
}

handlers! {
    initialize,
    increment
}

pub fn increment(InitContext { system }: InitContext) -> Result<(), ProgramError> {
    Ok(())
}

pub fn initialize() -> Result<(), ProgramError> {
    Ok(())
}

pub struct Counter {
    pub count: u64,
}

impl Owner for Counter {
    fn owner() -> pinocchio::pubkey::Pubkey {
        crate::ID
    }
}
