use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::reveal::*;
pub use crate::pray::location::*;

mod internal;
mod approval;
mod enumeration;
mod metadata;
mod mint;
mod nft_core;
mod royalty;
mod events;
mod reveal;
mod pray;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: UnorderedMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    //keep track of the royalty percentages for all tokens in a hash map
    pub royalty: HashMap<AccountId, u32>,

    pub collection_size: u32,
    pub collection_state: CollectionState,
    pub encrypted_metadata: Vector<String>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    EncryptedMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub enum CollectionState {
    Deployed,
    Published,
    Revealed,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(
        owner_id: AccountId,
        collection_size: u32,
        perpetual_royalties: Option<HashMap<AccountId, u32>>
    ) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Pray".to_string(),
                symbol: "PRAY".to_string(),
                icon: None,
                base_uri: Some("https://ipfs.io/ipfs/QmQkbuNeVoGhxsPNXpSx2mJyDHY3tL6T4Q3BemdtknQaKT".to_string()),
                reference: None,
                reference_hash: None,
            },
            perpetual_royalties,
            collection_size,
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(
        owner_id: AccountId,
        metadata: NFTContractMetadata,
        perpetual_royalties: Option<HashMap<AccountId, u32>>,
        collection_size: u32
    ) -> Self {
        // create a royalty map to store in the token
        let mut royalty = HashMap::new();

        // if perpetual royalties were passed into the function: 
        if let Some(perpetual_royalties) = perpetual_royalties {
            //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
            assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

            //iterate through the perpetual royalties and insert the account and amount in the royalty map
            for (account, amount) in perpetual_royalties {
                royalty.insert(account, amount);
            }
        }

        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: UnorderedMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            royalty: royalty,
            collection_size: collection_size,
            collection_state: CollectionState::Deployed,
            encrypted_metadata: Vector::new(StorageKey::EncryptedMetadata.try_to_vec().unwrap()),
        };

        //return the Contract object
        this
    }

    pub fn set_collection_state(&mut self, collection_state: CollectionState) {
        self.assert_called_by_owner();
        assert!(collection_state > self.collection_state, "Illegal state");

        self.collection_state = collection_state;
    }

    pub fn drop_state(&mut self) {
        self.assert_called_by_owner();
        // assert!(self.collection_state < CollectionState::Published, "Can't drop the state of a published collection");

        self.tokens_by_id.clear();
        self.token_metadata_by_id.clear();
        self.encrypted_metadata.clear();
    }
}

#[cfg(test)]
mod tests;