use anchor_lang::prelude::*;

declare_id!("5Kec3XAQJ8NC6CkRbSigxzDr67b8UEnqsURSQMsZVgve");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
