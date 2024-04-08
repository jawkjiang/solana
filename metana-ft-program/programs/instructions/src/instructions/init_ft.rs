use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3},
    token::{Mint, Token},
};
use mpl_token_metadata::types::DataV2;
use crate::utils::init_token_params::InitTokenParams;
use crate::state::ft_price::Price;
#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct InitFT<'info> {
    
    #[account(init, space = 8 + Price::INIT_SPACE, payer = payer, seeds = [( params.id.clone() + "-price" ).as_bytes().as_ref()], bump,)]
    ft_price: Account<'info,Price>,


    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    #[account(init, seeds = [params.id.clone().as_bytes().as_ref()], bump, payer = payer, mint::decimals = params.decimals, mint::authority = mint,)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
}

pub fn init_token(ctx: Context<InitFT>, metadata: InitTokenParams) -> Result<()> {

    let price_account = &mut ctx.accounts.ft_price;
    price_account.price = metadata.price;
    price_account.id = metadata.id.clone();

    // PDA seeds and bump to "sign" for CPI
    let binding = metadata.id.clone();
    let seeds = binding.as_bytes();
    let bump = ctx.bumps.mint;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];
    let data_v2 = DataV2 {
        name: metadata.name,
        symbol: metadata.symbol,
        uri: metadata.uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            // the metadata account being created
            metadata: ctx.accounts.metadata.to_account_info(),
            // the mint account of the metadata account
            mint: ctx.accounts.mint.to_account_info(),
            // the mint authority of the mint account
            mint_authority: ctx.accounts.mint.to_account_info(),
            // the update authority of the metadata account
            update_authority: ctx.accounts.mint.to_account_info(),
            // the payer for creating the metadata account
            payer: ctx.accounts.payer.to_account_info(),
            // the system program account
            system_program: ctx.accounts.system_program.to_account_info(),
            // the rent sysvar account
            rent: ctx.accounts.rent.to_account_info(),
        },
        signer,
    );

    create_metadata_accounts_v3(
        cpi_ctx, // cpi context
        data_v2, // token metadata
        true,    // is_mutable
        true,    // update_authority_is_signer
        None,    // collection details
    )?;
    Ok(())
}