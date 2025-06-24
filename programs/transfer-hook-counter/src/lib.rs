use anchor_lang::prelude::*;

use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::{
    instruction::{
        ExecuteInstruction, InitializeExtraAccountMetaListInstruction
    },
};

pub mod state;
pub mod instructions;

pub use instructions::*;

declare_id!("8GMnzdhvPv6R1eW7PrrmgwitGReF31dLrnspwSoCgQ1s");

#[program]
pub mod transfer_hook_counter {
    use spl_tlv_account_resolution::state::ExtraAccountMetaList;
    use spl_transfer_hook_interface::instruction::ExecuteInstruction;

    use super::*;

    #[instruction(discriminator = InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Initialize the counter account
        ctx.accounts.initialize(ctx.bumps)?;

        // Get the extra account metas for the transfer hook
        let extra_account_metas = Initialize::extra_account_metas()?;

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas
        )?;

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
}
