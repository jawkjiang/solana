use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct Price {
    #[max_len(50)]
    pub id: String,
    pub price: u64,  //solprice unit lamport
}