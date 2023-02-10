//! RMRK Base implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#![allow(clippy::inline_fn_without_body)]

pub mod internal;
pub mod traits;

use internal::Internal;

use rmrk_common::{
    errors::Result,
    utils::Utils,
};

use ink_prelude::string::{
    String as PreludeString,
    ToString,
};
use ink_storage::Mapping;

use openbrush::{
    contracts::{
        ownable::*,
        psp34::extensions::{
            enumerable::*,
            metadata::*,
        },
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        AccountId,
        Balance,
        Storage,
        String,
    },
};

use traits::{
    Minting,
    MintingLazy,
};

pub const STORAGE_MINTING_KEY: u32 = openbrush::storage_unique_key!(MintingData);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_MINTING_KEY)]
pub struct MintingData {
    pub last_token_id: u64,
    pub max_supply: u64,
    pub price_per_mint: Balance,
    pub nft_metadata: Mapping<Id, String>,
}

impl<T> Minting for T
where
    T: Storage<MintingData>
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<ownable::Data>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal
        + Utils,
{
    /// Mint one token to the specified account
    #[modifiers(only_owner)]
    default fn mint(&mut self, to: AccountId) -> Result<Id> {
        self._check_amount(1)?;
        self._mint(to)
    }

    /// Mint one or more tokens to the specified account
    #[modifiers(only_owner, non_reentrant)]
    default fn mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)> {
        self._check_amount(mint_amount)?;
        self._mint_many(to, mint_amount)
    }

    /// Assign metadata to specified token
    #[modifiers(only_owner)]
    default fn assign_metadata(&mut self, token_id: Id, metadata: PreludeString) -> Result<()> {
        self.data::<MintingData>()
            .nft_metadata
            .insert(token_id, &String::from(metadata));
        return Ok(())
    }

    /// Get max supply of tokens
    default fn max_supply(&self) -> u64 {
        self.data::<MintingData>().max_supply
    }

    /// Get URI for the token Id
    default fn token_uri(&self, token_id: u64) -> Result<PreludeString> {
        self.ensure_exists_and_get_owner(&Id::U64(token_id))?;
        let uri: PreludeString;
        match self
            .data::<MintingData>()
            .nft_metadata
            .get(Id::U64(token_id))
        {
            Some(token_uri) => {
                uri = PreludeString::from_utf8(token_uri).unwrap();
            }
            None => {
                let value = self.get_attribute(
                    self.data::<psp34::Data<enumerable::Balances>>()
                        .collection_id(),
                    String::from("baseUri"),
                );
                let token_uri = PreludeString::from_utf8(value.unwrap()).unwrap();
                uri = token_uri + &token_id.to_string() + &PreludeString::from(".json");
            }
        }
        Ok(uri)
    }
}

impl<T> MintingLazy for T
where
    T: Storage<MintingData>
        + Minting
        + Storage<psp34::Data<enumerable::Balances>>
        + Storage<reentrancy_guard::Data>
        + Storage<metadata::Data>
        + psp34::Internal
        + Utils,
{
    /// Mint one token to caller
    default fn mint_to_caller(&mut self) -> Result<()> {
        self._check_value(Self::env().transferred_value(), 1)?;
        self._mint(Self::env().caller())?;
        return Ok(())
    }

    /// Mint one or more tokens to caller
    #[modifiers(non_reentrant)]
    default fn mint_many_to_caller(&mut self, mint_amount: u64) -> Result<()> {
        self._check_value(Self::env().transferred_value(), mint_amount)?;
        self._mint_many(Self::env().caller(), mint_amount)?;
        Ok(())
    }

    /// Get token mint price
    default fn price(&self) -> Balance {
        self.data::<MintingData>().price_per_mint
    }
}
