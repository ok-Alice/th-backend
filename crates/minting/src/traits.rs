//! RMRK minting traits

use rmrk_common::errors::Result;

use ink_prelude::string::String as PreludeString;
use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        AccountId,
        Balance,
    },
};

#[openbrush::wrapper]
pub type MintingRef = dyn Minting;

#[openbrush::wrapper]
pub type MintingLazyRef = dyn MintingLazy;

/// Trait definitions for Minting functions
#[openbrush::trait_definition]
pub trait Minting {
    /// Mint one or more tokens.
    #[ink(message, payable)]
    fn mint(&mut self, to: AccountId) -> Result<Id>;

    /// Mint one or more tokens to specified account
    #[ink(message, payable)]
    fn mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)>;

    /// Assign metadata to specified token
    #[ink(message)]
    fn assign_metadata(&mut self, token_id: Id, metadata: PreludeString) -> Result<()>;

    /// Get max supply of tokens.
    #[ink(message)]
    fn max_supply(&self) -> u64;

    /// Get URI for the token Id.
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<PreludeString>;
}

#[openbrush::trait_definition]
pub trait MintingLazy {
    /// Mint one token to the caller
    #[ink(message, payable)]
    fn mint_to_caller(&mut self) -> Result<()>;

    /// Mint one or more tokens to the caller
    #[ink(message, payable)]
    fn mint_many_to_caller(&mut self, mint_amount: u64) -> Result<()>;

    /// Get token mint price.
    #[ink(message)]
    fn price(&self) -> Balance;
}
