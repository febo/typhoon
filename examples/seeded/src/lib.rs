#![no_std]

use {
    bytemuck::{AnyBitPattern, NoUninit},
    typhoon::prelude::*,
};

program_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

nostd_panic_handler!();
no_allocator!();

handlers! {
    initialize,
    increment,
}

#[context]
#[args(admin: Pubkey, bump: u8)]
pub struct InitContext {
    pub payer: Mut<Signer>,
    #[constraint(
        init,
        payer = payer,
        space = Counter::SPACE,
        seeded = [&args.admin],
        bump
    )]
    pub counter: Mut<Account<Counter>>,
    pub system: Program<System>,
}

#[context]
pub struct IncrementContext {
    pub admin: Signer,
    #[constraint(seeded, bump = counter.data()?.bump, has_one = admin @ProgramError::IllegalOwner)]
    pub counter: Mut<Account<Counter>>,
}

pub fn initialize(ctx: InitContext) -> ProgramResult {
    *ctx.counter.mut_data()? = Counter {
        admin: ctx.args.admin,
        count: 0,
        _padding: [0; 7],
        bump: ctx.bumps.counter,
    };

    Ok(())
}

pub fn increment(ctx: IncrementContext) -> ProgramResult {
    ctx.counter.mut_data()?.count += 1;

    Ok(())
}

#[derive(NoUninit, AnyBitPattern, AccountState, Copy, Clone)]
#[repr(C)]
#[no_space]
pub struct Counter {
    #[key]
    pub admin: Pubkey,
    pub bump: u8,
    _padding: [u8; 7],
    pub count: u64,
}

impl Counter {
    const SPACE: usize = 8 + core::mem::size_of::<Counter>();
}
