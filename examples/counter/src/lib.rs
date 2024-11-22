use crayfish_account_macro::account;
use crayfish_accounts::{Program, System};
use crayfish_context_macro::context;
use crayfish_handler_macro::handlers;
use crayfish_program_id_macro::program_id;
use crayfish_space::Space;
use pinocchio::{entrypoint, msg, program_error::ProgramError};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[context]
pub struct InitContext<'a> {
    // #[account(init)]
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

#[account]
#[derive(Space)]
pub struct Counter {
    pub count: u64,
}
