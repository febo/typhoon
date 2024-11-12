use crayfish_accounts::SystemAccount;
use crayfish_context_macro::context;
use crayfish_handler_macro::handlers;
use pinocchio::{entrypoint, msg, program_error::ProgramError};
use pinocchio_pubkey::declare_id;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[context]
pub struct InitContext<'a> {
    pub system: SystemAccount<'a>,
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
