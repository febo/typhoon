#![no_std]

use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

nostd_panic_handler!();
no_allocator!();

#[repr(C)]
#[derive(Debug, PartialEq, AnyBitPattern, NoUninit, Copy, Clone)]
pub struct InitArgs {
    pub value: PodU64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, AnyBitPattern, NoUninit)]
#[repr(transparent)]
pub struct PodU64(pub [u8; 8]);

impl PodU64 {
    pub const fn from_primitive(n: u64) -> Self {
        Self(n.to_le_bytes())
    }
}
impl From<u64> for PodU64 {
    fn from(n: u64) -> Self {
        Self::from_primitive(n)
    }
}
impl From<PodU64> for u64 {
    fn from(pod: PodU64) -> Self {
        Self::from_le_bytes(pod.0)
    }
}

#[context]
#[args(InitArgs)]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        space = Buffer::SPACE
    )]
    pub buffer: Mut<Account<Buffer>>,
    pub system: Program<System>,
}

#[context]
#[args(value: PodU64, other_value: PodU64)]
pub struct SetValueContext {
    pub buffer: Mut<Account<Buffer>>,
}

handlers! {
    initialize,
    set_value,
    set_and_add_values,
}

pub fn initialize(ctx: InitContext) -> ProgramResult {
    ctx.buffer.mut_data()?.value1 = ctx.args.value.into();

    Ok(())
}

pub fn set_value(ctx: SetValueContext, more_args: Arg<PodU64>) -> ProgramResult {
    let mut data = ctx.buffer.mut_data()?;
    data.value1 = ctx.args.value.into();
    data.value2 = (*more_args).into();

    Ok(())
}

pub fn set_and_add_values(ctx_a: SetValueContext, ctx_b: SetValueContext) -> ProgramResult {
    let value_a = ctx_a.args.value.into();
    let value_b = ctx_b.args.value.into();
    ctx_a.buffer.mut_data()?.value1 = value_a;
    ctx_b.buffer.mut_data()?.value1 = value_b;
    ctx_a.buffer.mut_data()?.value2 = value_a + value_b;

    Ok(())
}

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub value1: u64,
    pub value2: u64,
}
