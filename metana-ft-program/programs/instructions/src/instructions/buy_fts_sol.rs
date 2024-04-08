use anchor_lang::{prelude::*,system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount,transfer,Transfer},
};

use crate::state::ft_price::Price;
use std::str::FromStr;

// Treasury Wallet for Bark
pub const RECIPIENT: &str = "66T11pRjNzMnvgAUYTo6zjRGqEJK5Gi8WkEaDVmwEYoa";
const LAMPORTS_PER_SOL: u64 = 1000000000;


#[derive(Accounts)]
pub struct BuyTokens<'info> {

    #[account( mut, seeds = [b"credit"],bump,mint::authority = mint_credit,)]
    pub mint_credit: Box<Account<'info, Mint>>,
    #[account(init_if_needed,payer = payer,associated_token::mint = mint_credit, associated_token::authority = payer,)]
    pub destination_credit: Box<Account<'info, TokenAccount>>,
    

    #[account( mut, seeds = [b"00001"],bump,mint::authority = mint1,)]
    pub mint1: Box<Account<'info, Mint>>,
    #[account(init_if_needed,payer = payer,associated_token::mint = mint1, associated_token::authority = payer,)]
    pub destination1: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    ft_price1: Box<Account<'info,Price>>,

    #[account( mut, seeds = [b"00002"],bump,mint::authority = mint2,)]
    pub mint2: Box<Account<'info, Mint>>,
    #[account(init_if_needed,payer = payer,associated_token::mint = mint2, associated_token::authority = payer,)]
    pub destination2: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    ft_price2: Box<Account<'info,Price>>,

    #[account( mut, seeds = [b"00003"],bump,mint::authority = mint3,)]
    pub mint3: Box<Account<'info, Mint>>,
    #[account(init_if_needed,payer = payer,associated_token::mint = mint3, associated_token::authority = payer,)]
    pub destination3: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    ft_price3: Box<Account<'info,Price>>,

    // #[account( mut, seeds = [b"00004"],bump,mint::authority = mint4,)]
    // pub mint4: Account<'info, Mint>,
    // #[account(init_if_needed,payer = payer,associated_token::mint = mint4, associated_token::authority = payer,)]
    // pub destination4: Account<'info, TokenAccount>,
    // #[account(mut)]
    // ft_price4: Account<'info,Price>,

    // #[account( mut, seeds = [b"00005"],bump,mint::authority = mint5,)]
    // pub mint5: Account<'info, Mint>,
    // #[account(init_if_needed,payer = payer,associated_token::mint = mint5, associated_token::authority = payer,)]
    // pub destination5: Account<'info, TokenAccount>,
    // #[account(mut)]
    // ft_price5: Account<'info,Price>,

    // #[account( mut, seeds = [b"00006"],bump,mint::authority = mint6,)]
    // pub mint6: Account<'info, Mint>,
    // #[account(init_if_needed,payer = payer,associated_token::mint = mint6, associated_token::authority = payer,)]
    // pub destination6: Account<'info, TokenAccount>,
    // #[account(mut)]
    // ft_price6: Account<'info,Price>,

    // #[account( mut, seeds = [b"00007"],bump,mint::authority = mint7,)]
    // pub mint7: Account<'info, Mint>,
    // #[account(init_if_needed,payer = payer,associated_token::mint = mint7, associated_token::authority = payer,)]
    // pub destination7: Account<'info, TokenAccount>,
    // #[account(mut)]
    // ft_price7: Account<'info,Price>,

    // #[account( mut, seeds = [b"00008"],bump,mint::authority = mint8,)]
    // pub mint8: Account<'info, Mint>,
    // #[account(init_if_needed,payer = payer,associated_token::mint = mint8, associated_token::authority = payer,)]
    // pub destination8: Account<'info, TokenAccount>,
    // #[account(mut)]
    // ft_price8: Account<'info,Price>,

    // #[account( mut, seeds = [b"00009"],bump,mint::authority = mint9,)]
    // pub mint9: Account<'info, Mint>,
    // #[account(init_if_needed,payer = payer,associated_token::mint = mint9, associated_token::authority = payer,)]
    // pub destination9: Account<'info, TokenAccount>,
    // #[account(mut)]
    // ft_price9: Account<'info,Price>,

    #[account(mut)]
    pub payer: Signer<'info>,
     #[account(
        mut,
        constraint = Pubkey::from_str(RECIPIENT).unwrap() == *sol_recipient.key,
    )]
    pub sol_recipient: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_credit,
        associated_token::authority = sol_recipient,
    )]
    pub credit_recipient:Box<Account<'info, TokenAccount>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}




pub fn mint_tokens(ctx: Context<BuyTokens>, quantitys: [u64;3], use_credit: bool, swap_credits: u64) -> Result<()> {

        let mut transfer_amount = 0;
        

        let credit_seeds = &["credit".as_bytes(), &[ctx.bumps.mint_credit]];
        let credit_signer = [&credit_seeds[..]];

        if quantitys[0] > 0{
            let seeds = &["00001".as_bytes(), &[ctx.bumps.mint1]];
            let signer = [&seeds[..]];

            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        authority: ctx.accounts.mint1.to_account_info(),
                        to: ctx.accounts.destination1.to_account_info(),
                        mint: ctx.accounts.mint1.to_account_info(),
                    },
                    &signer,
                ),
                quantitys[0],
            )?;

            transfer_amount += ctx.accounts.ft_price1.price * quantitys[0];
        }

        if quantitys[1] > 0{
            let seeds = &["00002".as_bytes(), &[ctx.bumps.mint2]];
            let signer = [&seeds[..]];

            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        authority: ctx.accounts.mint2.to_account_info(),
                        to: ctx.accounts.destination2.to_account_info(),
                        mint: ctx.accounts.mint2.to_account_info(),
                    },
                    &signer,
                ),
                quantitys[1],
            )?;
            transfer_amount += ctx.accounts.ft_price2.price * quantitys[1];
        }

        msg!("Minting tokens to associated token account...");

        if quantitys[2] > 0{
            let seeds = &["00003".as_bytes(), &[ctx.bumps.mint3]];
            let signer = [&seeds[..]];

            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        authority: ctx.accounts.mint3.to_account_info(),
                        to: ctx.accounts.destination3.to_account_info(),
                        mint: ctx.accounts.mint3.to_account_info(),
                    },
                    &signer,
                ),
                quantitys[2],
            )?;

            transfer_amount += ctx.accounts.ft_price3.price * quantitys[2];
        }

        
        // if quantitys[3] > 0{
        //     let seeds = &["00004".as_bytes(), &[ctx.bumps.mint4]];
        //     let signer = [&seeds[..]];

        //     mint_to(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             MintTo {
        //                 authority: ctx.accounts.mint4.to_account_info(),
        //                 to: ctx.accounts.destination4.to_account_info(),
        //                 mint: ctx.accounts.mint4.to_account_info(),
        //             },
        //             &signer,
        //         ),
        //         quantitys[3],
        //     )?;
        //     transfer_amount += ctx.accounts.ft_price4.price * quantitys[3];
        // }

        // if quantitys[4] > 0{
        //     let seeds = &["00005".as_bytes(), &[ctx.bumps.mint5]];
        //     let signer = [&seeds[..]];

        //     mint_to(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             MintTo {
        //                 authority: ctx.accounts.mint5.to_account_info(),
        //                 to: ctx.accounts.destination5.to_account_info(),
        //                 mint: ctx.accounts.mint5.to_account_info(),
        //             },
        //             &signer,
        //         ),
        //         quantitys[4],
        //     )?;
        //     transfer_amount += ctx.accounts.ft_price5.price * quantitys[4];
        // }

        // if quantitys[5] > 0{
        //     let seeds = &["00006".as_bytes(), &[ctx.bumps.mint6]];
        //     let signer = [&seeds[..]];

        //     mint_to(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             MintTo {
        //                 authority: ctx.accounts.mint6.to_account_info(),
        //                 to: ctx.accounts.destination6.to_account_info(),
        //                 mint: ctx.accounts.mint6.to_account_info(),
        //             },
        //             &signer,
        //         ),
        //         quantitys[5],
        //     )?;
        //     transfer_amount += ctx.accounts.ft_price6.price * quantitys[5];
        // }
        // if quantitys[6] > 0{
        //     let seeds = &["00007".as_bytes(), &[ctx.bumps.mint7]];
        //     let signer = [&seeds[..]];

        //     mint_to(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             MintTo {
        //                 authority: ctx.accounts.mint7.to_account_info(),
        //                 to: ctx.accounts.destination7.to_account_info(),
        //                 mint: ctx.accounts.mint7.to_account_info(),
        //             },
        //             &signer,
        //         ),
        //         quantitys[7],
        //     )?;
        //     transfer_amount += ctx.accounts.ft_price7.price * quantitys[6];
        // }

        // if quantitys[7] > 0{
        //     let seeds = &["00008".as_bytes(), &[ctx.bumps.mint8]];
        //     let signer = [&seeds[..]];

        //     mint_to(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             MintTo {
        //                 authority: ctx.accounts.mint8.to_account_info(),
        //                 to: ctx.accounts.destination8.to_account_info(),
        //                 mint: ctx.accounts.mint8.to_account_info(),
        //             },
        //             &signer,
        //         ),
        //         quantitys[7],
        //     )?;
        //     transfer_amount += ctx.accounts.ft_price8.price * quantitys[7];
        // }

        // if quantitys[8] > 0{
        //     let seeds = &["00009".as_bytes(), &[ctx.bumps.mint9]];
        //     let signer = [&seeds[..]];

        //     mint_to(
        //         CpiContext::new_with_signer(
        //             ctx.accounts.token_program.to_account_info(),
        //             MintTo {
        //                 authority: ctx.accounts.mint9.to_account_info(),
        //                 to: ctx.accounts.destination9.to_account_info(),
        //                 mint: ctx.accounts.mint9.to_account_info(),
        //             },
        //             &signer,
        //         ),
        //         quantitys[8],
        //     )?;
        //     transfer_amount += ctx.accounts.ft_price9.price * quantitys[8];
        // }

        msg!(
                "transfer sol amount beofre using credit: {}",
                transfer_amount.clone()
        );

        if use_credit{

            let mut credit_balance = ctx.accounts.destination_credit.amount;
             msg!(
                "payer credits before: {}",
                credit_balance.clone()
            );

            if swap_credits > 0 {
                let lamport_amount = swap_credits *  100000;

                system_program::transfer(
                    CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        system_program::Transfer {
                            from: ctx.accounts.payer.to_account_info(),
                            to: ctx.accounts.sol_recipient.to_account_info(),
                        },
                    ),
                    lamport_amount,
                )?;

                mint_to(
                    CpiContext::new_with_signer(
                        ctx.accounts.token_program.to_account_info(),
                        MintTo {
                            authority: ctx.accounts.mint_credit.to_account_info(),
                            to: ctx.accounts.destination_credit.to_account_info(),
                            mint: ctx.accounts.mint_credit.to_account_info(),
                        },
                        &credit_signer,
                    ),
                    swap_credits,
                )?;

                credit_balance += swap_credits;
            }


            let credit_balance_after = ctx.accounts.destination_credit.amount;
             msg!(
                "payer credits after : {}",
                credit_balance_after.clone()
            );

            // require!((transfer_amount as f64) * 0.3 < (credit_balance as f64) * 0.0002 * (LAMPORTS_PER_SOL as f64) , CreditError::Exceed);

            let mut can_used_credits = credit_balance;
            if (transfer_amount as f64) * 0.3 < (credit_balance as f64) * 0.0002 * (LAMPORTS_PER_SOL as f64) {
                can_used_credits = (
                    (
                        (transfer_amount as f64) * 0.3 / 
                        (0.0002 * (LAMPORTS_PER_SOL as f64))
                    ) as u64
                    
                );
            }


            msg!(
                "can_used_credits : {}",
                can_used_credits.clone()
            );

            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.destination_credit.to_account_info(),
                        to: ctx.accounts.credit_recipient.to_account_info(),
                        authority: ctx.accounts.payer.to_account_info(),
                    },
                ),
                can_used_credits, // Transfer amount, adjust for decimals
            )?;

            transfer_amount = transfer_amount - can_used_credits * 2 * 100000 ; 

        }
        

        msg!(
            "transfer sol amount after using credit: {}",
            transfer_amount.clone()
        );


        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.sol_recipient.to_account_info(),
                },
            ),
            transfer_amount,
        )?;

        mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        authority: ctx.accounts.mint_credit.to_account_info(),
                        to: ctx.accounts.destination_credit.to_account_info(),
                        mint: ctx.accounts.mint_credit.to_account_info(),
                    },
                    &credit_signer,
                ),
                transfer_amount / 10000000 ,
        )?; 
        






        Ok(())
}


// #[error_code]
// pub enum CreditError {
//     #[msg("credits used exceeds the max usage amount")]
//     Exceed
// }