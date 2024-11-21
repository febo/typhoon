use crayfish_program::system_program;

use crate::ProgramId;

pub struct System;

impl ProgramId for System {
    const ID: crayfish_program::Pubkey = system_program::ID;
}
