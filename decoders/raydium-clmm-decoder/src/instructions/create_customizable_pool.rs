use carbon_core::{borsh, CarbonDeserialize};

// No public IDL; data-derived layout: sqrt_price_x64 then a 2-byte tail left as tolerated trailing bytes.
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2b44d4a7592fa401")]
pub struct CreateCustomizablePool {
    pub sqrt_price_x64: u128,
}

#[derive(Debug, PartialEq)]
pub struct CreateCustomizablePoolInstructionAccounts {
    pub pool_creator: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub token_mint0: solana_pubkey::Pubkey,
    pub token_mint1: solana_pubkey::Pubkey,
    pub token_vault0: solana_pubkey::Pubkey,
    pub token_vault1: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
    pub tick_array_bitmap: solana_pubkey::Pubkey,
    pub token_program0: solana_pubkey::Pubkey,
    pub token_program1: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCustomizablePool {
    type ArrangedAccounts = CreateCustomizablePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        // Same leading account order as CreatePool; the variant's one extra trailing account rides in _remaining.
        let [pool_creator, amm_config, pool_state, token_mint0, token_mint1, token_vault0, token_vault1, observation_state, tick_array_bitmap, token_program0, token_program1, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateCustomizablePoolInstructionAccounts {
            pool_creator: pool_creator.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            token_mint0: token_mint0.pubkey,
            token_mint1: token_mint1.pubkey,
            token_vault0: token_vault0.pubkey,
            token_vault1: token_vault1.pubkey,
            observation_state: observation_state.pubkey,
            tick_array_bitmap: tick_array_bitmap.pubkey,
            token_program0: token_program0.pubkey,
            token_program1: token_program1.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
