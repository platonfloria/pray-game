use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, ext_contract, AccountId, BlockHeight, PanicOnDefault, Promise, PromiseOrValue, PromiseResult, Gas};
use near_sdk::collections::{UnorderedMap};
pub type TokenId = String;

const GAS_FOR_CHARACTER_MOVE: Gas = Gas(1_000_000_000_000);
const GAS_FOR_RESOLVE_CHARACTER_MOVE: Gas = Gas(1_000_000_000_000);

mod internal;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    name: String, // Location name
    rate: u32, // Base resource collection/crafting rate
    present_characters: UnorderedMap<TokenId, CharacterData>,
}

#[derive(BorshSerialize)]
pub enum StorageKey {
    PresentCharacters,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct CharacterData {
    pub owner: AccountId,
    pub entered_at_block: BlockHeight,
}

#[ext_contract(ext_character)]
pub trait ExtCharacter {
    fn move_character(character_id: &TokenId, destination: Option<String>) -> Promise;
}

#[ext_contract(ext_self)]
trait LocationResolver {
    fn resolve_character_move(
        &mut self,
        character_id: &TokenId,
        enter: bool
    ) -> bool;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        name: String,
        rate: u32
    ) -> Self {
        Self {
            owner_id,
            name,
            rate,
            present_characters: UnorderedMap::new(StorageKey::PresentCharacters.try_to_vec().unwrap())
        }
    }

    pub fn set_rate(&mut self, rate: u32) {
        self.assert_called_by_owner();
        self.rate = rate;
    }

    pub fn enter(&mut self, character_id: TokenId) -> PromiseOrValue<bool> {
        ext_character::ext(AccountId::try_from("collection.pray.devgenerate.testnet".to_string()).unwrap())
            .with_static_gas(GAS_FOR_CHARACTER_MOVE)
            .move_character(
                &character_id,
                Some(self.name.clone())
            )
        .then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_CHARACTER_MOVE)
                .resolve_character_move(
                    &character_id,
                    true
                )
        ).into()
    }

    pub fn leave(&mut self, character_id: TokenId) -> PromiseOrValue<bool> {
        ext_character::ext(AccountId::try_from("collection.pray.devgenerate.testnet".to_string()).unwrap())
            .with_static_gas(GAS_FOR_CHARACTER_MOVE)
            .move_character(
                &character_id,
                None
            )
        .then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_RESOLVE_CHARACTER_MOVE)
                .resolve_character_move(
                    &character_id,
                    false
                )
        ).into()
    }
}

#[near_bindgen]
impl LocationResolver for Contract {
    #[private]
    fn resolve_character_move(
        &mut self,
        character_id: &TokenId,
        enter: bool
    ) -> bool {
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            if let Ok(moved) = near_sdk::serde_json::from_slice::<bool>(&value) {
                if moved {
                    if enter {
                        self.present_characters.insert(&character_id, &CharacterData {
                            owner: env::signer_account_id(),
                            entered_at_block: env::block_height(),
                        });
                    } else {
                        return match self.present_characters.remove(&character_id) {
                            Some(_) => true,
                            None => false
                        };
                    }
                    return true;
                }
            }
        }
        false
    }
}