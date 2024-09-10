use anchor_lang::prelude::*;

declare_id!("B6uxkHNw5FMMKt88S1MU6gy8us6juKSwHiGvGw4NA1ri");

#[program]
pub mod some_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        /*
        

         */
        Ok(())
    }
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        Ok(())
    }
    pub fn get_rewards(ctx: Context<GetRewards>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Stake {}

#[derive(Accounts)]
pub struct Unstake {}

#[derive(Accounts)]
pub struct GetRewards {}