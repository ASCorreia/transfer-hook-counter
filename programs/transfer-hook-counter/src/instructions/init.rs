use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_2022::Token2022, 
    token_interface::Mint
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta,
    seeds::Seed,
    state::ExtraAccountMetaList,
};

use crate::state::Counter;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Counter::INIT_SPACE,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub mint: InterfaceAccount<'info, Mint>,
    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            Initialize::extra_account_metas()?.len()
        )?,
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        self.counter.count = 0;
        self.counter.bump = bumps.counter;

        Ok(())
    }

    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(
            vec![
                ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::Literal {
                            bytes: b"counter".to_vec(),
                        },
                    ],
                    false, // is_signer
                    true // is_writable
                )?
            ]
        )
    }
}
