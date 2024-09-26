use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub owner: Pubkey,
    #[max_len(20)]
    pub name: String,
    #[max_len(2000)]
    pub avatar: String,
    pub last_post_id: u8,
    pub post_count: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PostAccount {
    pub id: u8,
    #[max_len(20)]
    pub title: String,
    #[max_len(2000)]
    pub content: String,
    pub user: Pubkey,
    pub owner: Pubkey,
}
