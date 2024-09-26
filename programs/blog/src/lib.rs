use anchor_lang::prelude::*;

mod constant;
mod state;
use crate::{constant::*, state::*};

declare_id!("HCMUbrXqGLjERGVVZFmkBkffNVdGRstdQpmEu1qPCgaV");

#[program]
pub mod blog {
    use super::*;

    pub fn init_user(ctx: Context<InitUser>, name: String, avatar: String) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.avatar = avatar;
        user_account.last_post_id = 0;
        user_account.post_count = 0;
        user_account.owner = ctx.accounts.owner.key();

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;

        post_account.id = user_account.last_post_id;
        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.owner = ctx.accounts.owner.key();

        user_account.last_post_id = user_account.last_post_id.checked_add(1).unwrap();
        user_account.post_count = user_account.post_count.checked_add(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitUser<'info> {
    #[account(
        init,
        seeds=[USER_SEED,owner.key().as_ref()],
        bump,
        payer=owner,
        space =  8 + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]

pub struct CreatePost<'info> {
    #[account(
        init,
        seeds=[POST_SEED,owner.key().as_ref(),&user_account.last_post_id.to_le_bytes() ],
        bump,
        payer=owner,
        space =  8 + PostAccount::INIT_SPACE,
    )]
    pub post_account: Account<'info, PostAccount>,

    #[account(
        mut,
        seeds = [USER_SEED,owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
