use anchor_lang::prelude::*;

declare_id!("HdwjjEUfrFF8G66oAa7Pymaa4DHwR2QsdVPfMWTnvMXE");

#[program]
pub mod anchor_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
