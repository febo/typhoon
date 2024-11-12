use crayfish_program::{program_error::ProgramError, RawAccountInfo};

use crate::HandlerContext;

impl<'a> HandlerContext<'a> for &'a [RawAccountInfo] {
    fn from_entrypoint(
        accounts: &mut &'a [RawAccountInfo],
        _instruction_data: &mut &'a [u8],
    ) -> Result<Self, ProgramError> {
        Ok(accounts)
    }
}
