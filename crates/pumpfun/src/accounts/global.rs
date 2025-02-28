use anchor_client::solana_sdk::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

/// Represents the global configuration account for token pricing and fees
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct GlobalAccount {
    /// Unique identifier for the global account
    pub discriminator: u64,
    /// Whether the global account has been initialized
    pub initialized: bool,
    /// Authority that can modify global settings (存储为字节数组)
    pub authority_bytes: [u8; 32],
    /// Account that receives fees (存储为字节数组)
    pub fee_recipient_bytes: [u8; 32],
    /// Initial virtual token reserves for price calculations
    pub initial_virtual_token_reserves: u64,
    /// Initial virtual SOL reserves for price calculations  
    pub initial_virtual_sol_reserves: u64,
    /// Initial actual token reserves available for trading
    pub initial_real_token_reserves: u64,
    /// Total supply of tokens
    pub token_total_supply: u64,
    /// Fee in basis points (1/100th of a percent)
    pub fee_basis_points: u64,
}

impl GlobalAccount {
    /// Creates a new global account instance
    ///
    /// # Arguments
    /// * `discriminator` - Unique identifier for the account
    /// * `initialized` - Whether the account is initialized
    /// * `authority` - Authority pubkey that can modify settings
    /// * `fee_recipient` - Account that receives fees
    /// * `initial_virtual_token_reserves` - Initial virtual token reserves
    /// * `initial_virtual_sol_reserves` - Initial virtual SOL reserves
    /// * `initial_real_token_reserves` - Initial actual token reserves
    /// * `token_total_supply` - Total supply of tokens
    /// * `fee_basis_points` - Fee in basis points
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        discriminator: u64,
        initialized: bool,
        authority: Pubkey,
        fee_recipient: Pubkey,
        initial_virtual_token_reserves: u64,
        initial_virtual_sol_reserves: u64,
        initial_real_token_reserves: u64,
        token_total_supply: u64,
        fee_basis_points: u64,
    ) -> Self {
        Self {
            discriminator,
            initialized,
            authority_bytes: authority.to_bytes(),
            fee_recipient_bytes: fee_recipient.to_bytes(),
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
        }
    }

    /// Get the authority pubkey
    pub fn authority(&self) -> Pubkey {
        Pubkey::new_from_array(self.authority_bytes)
    }

    /// Get the fee recipient pubkey
    pub fn fee_recipient(&self) -> Pubkey {
        Pubkey::new_from_array(self.fee_recipient_bytes)
    }

    /// Calculates the initial amount of tokens received for a given SOL amount
    ///
    /// # Arguments
    /// * `amount` - Amount of SOL to spend
    ///
    /// # Returns
    /// Amount of tokens that would be received
    pub fn get_initial_buy_price(&self, amount: u64) -> u64 {
        if amount == 0 {
            return 0;
        }

        let n: u128 = (self.initial_virtual_sol_reserves as u128)
            * (self.initial_virtual_token_reserves as u128);
        let i: u128 = (self.initial_virtual_sol_reserves as u128) + (amount as u128);
        let r: u128 = n / i + 1;
        let s: u128 = (self.initial_virtual_token_reserves as u128) - r;

        if s < (self.initial_real_token_reserves as u128) {
            s as u64
        } else {
            self.initial_real_token_reserves
        }
    }
}
