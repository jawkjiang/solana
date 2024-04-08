use anchor_lang::prelude::*;

declare_id!("672VeCnpAxhkd9TueCo8653hiYXP8kJRXbhPzi3YWZor");


use instructions::*;
mod instructions;
mod state;
mod utils;
use utils::init_token_params::InitTokenParams;


#[program]
pub mod metana_nft {
    use super::*;
    pub fn init_ft(ctx: Context<InitFT>, metadata: InitTokenParams) -> Result<()> {
        init_ft::init_token(ctx,metadata)
    }

    pub fn buy_ft(ctx: Context<BuyTokens>, quantitys: [u64;3],use_credit: bool, swap_credits: u64) -> Result<()> {
        buy_fts_sol::mint_tokens(ctx,quantitys,use_credit,swap_credits)
    }


}

