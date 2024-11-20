use crayfish_accounts::{Owner, ProgramId, SystemAccount};
use crayfish_context_macro::context;
use crayfish_handler_macro::handlers;
use pinocchio::{entrypoint, msg, program_error::ProgramError, pubkey::Pubkey};
use pinocchio_pubkey::declare_id;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub struct CounterProgram;

impl ProgramId for CounterProgram {
    const ID: Pubkey = crate::ID;
}

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

pub struct Counter {
    pub count: u64,
}

impl Owner for Counter {
    fn owner() -> pinocchio::pubkey::Pubkey {
        crate::ID
    }
}
