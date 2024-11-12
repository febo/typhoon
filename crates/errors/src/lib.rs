use crayfish_program::{msg, program_error::ProgramError, pubkey::Pubkey};
use num_traits::{FromPrimitive, ToPrimitive};
use thiserror::Error;

/// Maybe rework with thiserror 2.0
#[derive(Debug, Error)]
pub enum Error {
    #[error("TODO")]
    InvalidProgramExecutable,

    #[error("TODO")]
    AccountNotInitialized,

    // #[error("The owner of the account is not {wanted}, currently it's {current}")]
    #[error("TODO")]
    InvalidOwner { wanted: Pubkey, current: Pubkey },

    #[error("The given account is not mutable")]
    AccountNotMutable,

    #[error("TODO")]
    AccountNotSigner,

    #[error("TODO")]
    AccountOwnedByWrongProgram,

    #[error("TODO")]
    CannotDeserializeData,
}

impl FromPrimitive for Error {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            3000 => Some(Error::AccountNotInitialized),

            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::from_i64(n as i64)
    }
}

impl ToPrimitive for Error {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_i64().map(|n| n as u64)
    }
}

impl From<Error> for ProgramError {
    fn from(value: Error) -> Self {
        msg!("[ERROR] {}", value.to_string());
        ProgramError::Custom(value.to_u32().unwrap())
    }
}
