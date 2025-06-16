use core::mem::transmute;
use pinocchio::program_error::ProgramError;

#[repr(u8)]
#[derive(Clone, Debug)]
#[rustfmt::skip]
pub enum Instruction {
    Ping,
    Log,
    CreateAccount,
}

impl TryFrom<&[u8]> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let [discriminator, _remaining @ ..] = value else {
            return Err(ProgramError::InvalidInstructionData);
        };

        match *discriminator {
            0..=2 => Ok(unsafe { transmute::<u8, Instruction>(*discriminator) }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
