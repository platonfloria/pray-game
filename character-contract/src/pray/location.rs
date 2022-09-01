use crate::*;

#[near_bindgen]
impl Contract {
    pub fn move_character(&mut self, character_id: TokenId, destination: Option<String>) -> bool {
        let caller_id: AccountId = env::predecessor_account_id();
        assert_eq!(caller_id, AccountId::try_from("location.pray.devgenerate.testnet".to_string()).unwrap(), "Can only be called by location contract");

        let signer_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&character_id).expect("No token");
        assert_eq!(signer_id, token.owner_id, "Character is not owned by the signer");

        // check character's location

        let message = match destination {
            Some(location) => format!("Character {} entered {}", character_id, location),
            None => format!("Character {} left", character_id),
        };
        env::log_str(&message);
        
        true
    }
}