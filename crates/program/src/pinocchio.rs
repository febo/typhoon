use account_info::AccountInfo;

pub use account_info::{Ref, RefMut};
pub use pinocchio::*;
pub use pinocchio_system as system_program;

pub type RawAccountInfo = AccountInfo;
pub type Signer<'a, 'b> = instruction::Signer<'a, 'b>;
