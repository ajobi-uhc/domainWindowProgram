use anchor_lang::{prelude::*, accounts::program_account::ProgramAccount};

use anchor_spl::token::{self,Mint,TokenAccount, Token};
use std::str::FromStr;

use anchor_lang::solana_program::program_option::COption;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/*
Each ambience phase will have a ambience pool account and ambience ownership NFT
Each builder will have a builder_account
Every time a builder is added, a token is transferred to the ambience pool account and a redeemable is transferred
to the builder account
This redeemable represents that builders ownership in the pool,
when they withdraw funds, the redeemable is burned

When time limit for withdrawal is over, every redeemable is burned and the ambience ownership NFT is calculated and transferred
according to the redeemable tokens user owned

*/


/* IDO pool minting nfts 

When adding a builder, they deposit domain charged components
and I 

*/

#[program]
pub mod fractions {
    use anchor_spl::token::Transfer;

    use super::*;
    pub fn initialize(ctx: Context<Initialize>, 
        phase_number:u8,
        nonce: u8,
        nonce_for_pool:u8,
        num_pool_prize_tokens:u64,
        start_ambience_phase: i64,
        end_ambience_pool_token_limit:u64
    
    ) -> ProgramResult {

      let ambience_pool = &mut ctx.accounts.ambience_phase;

      ambience_pool.pool_prize_pubkey = ctx.accounts.pool_prize.key();
      ambience_pool.pool_component_pubkey = ctx.accounts.pool_component_account.key();
      ambience_pool.pool_signer = ctx.accounts.pool_signer.key();
      ambience_pool.num_components = 0;
      ambience_pool.start_ambience_phase = start_ambience_phase;
      ambience_pool.num_components_needed = end_ambience_pool_token_limit;
      ambience_pool.phase_number = phase_number;
      
    //initiate transfer of pool prize from user to Pool
    let cpi_accounts  = Transfer {
        from:ctx.accounts.user_prize_to_transfer.to_account_info(),
        to:ctx.accounts.pool_prize.to_account_info(),
        authority:ctx.accounts.user.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.clone();
    let cpi_ctx = CpiContext::new(cpi_program.to_account_info(),cpi_accounts);
    token::transfer(cpi_ctx, num_pool_prize_tokens)?;
        Ok(())
    }


    pub fn despoit_components(ctx:Context<AddBuilder>) -> ProgramResult {


        Ok(())
    }

   
    pub fn redeem_nfts(ctx: Context<RedeemNFT>) -> ProgramResult {
        Ok(())
    }


}

#[derive(Accounts)]
#[instruction(phase_number:u8, init_nonce:u8, init_pool_nonce:u8)]
pub struct Initialize<'info> {
    //Initialize a new ambient phase
    #[account(
        seeds = [b"signer"],
        bump = init_pool_nonce
    )]
    pool_signer: AccountInfo<'info>,
    #[account(mut,signer)]
    user:AccountInfo<'info>,
    //admin is distribution authority, owner of the prize, owner of redeemable mint, owner of usdc account
    #[account(
        init, 
        payer = user, 
 
        space= 240
     )]
     ambience_phase: Box<Account<'info,AmbiencePool>>,



     #[account(mut,constraint = pool_prize.owner == pool_signer.key())]
     pool_prize: Account<'info, TokenAccount>,

     #[account(constraint = pool_component_mint.mint_authority == COption :: Some(pool_signer.key()))]
     pool_component_mint:Account<'info,Mint>,

     

     #[account(constraint = pool_component_account.owner == pool_signer.key())]
     pool_component_account: Account<'info, TokenAccount>,

     #[account(mut,constraint = user_prize_to_transfer.owner == user.key())]
     user_prize_to_transfer: Account<'info, TokenAccount>,


     


     system_program: Program<'info, System>,
     token_program: Program<'info, Token>,
     rent: Sysvar<'info, Rent>



}

#[derive(Accounts)]
pub struct WithdrawNFT{

}

#[derive(Accounts)]
pub struct AddBuilder<'info> {
    //User submitting components
   
    #[account(mut)]
    builder: Signer<'info>,
    //Mint account of component
    #[account(constraint = component_mint.mint_authority == COption :: Some(Pubkey::from_str("3ANuEmA1Prg6STW7LUGCvc5NYZRbmGVhAvGGT7gPUUVg").unwrap()))]
    component_mint: Account<'info, Mint>,
    //Token account of user holding mint
    #[account(owner = builder.key(), constraint = component_token_account.mint == component_mint.key())]
    component_token_account: Account<'info, TokenAccount>,
    
    





}

#[derive(Accounts)]
pub struct RedeemNFT {

}

#[account]
pub struct AmbiencePool {

   pub  phase_number:u8,
   pub  pool_prize_pubkey: Pubkey,
    pub pool_component_pubkey: Pubkey,
    pub num_components: u64,
    pub pool_signer:Pubkey,
    pub start_ambience_phase:i64,
    pub num_components_needed:u64
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Builder {
     builder_pubkey: Pubkey,
     builder_stake:u32,
     builder_share: u32
}

#[account]
pub struct Stuff{}
