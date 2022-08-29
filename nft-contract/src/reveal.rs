use crate::*;
use serde_json;

use std::collections::HashMap;

#[near_bindgen]
impl Contract {
    pub fn append_encrypted_metadata(&mut self, encrypted_metadata: String) {
        self.assert_called_by_owner();
        assert!(self.collection_state < CollectionState::Published, "Can only append to the collection before it is published");

        self.encrypted_metadata.push(&encrypted_metadata);
    }

    pub fn reveal(&mut self, password: String) -> bool {
        self.assert_called_by_owner();
        assert!(self.collection_state >= CollectionState::Published, "Can't reveal metadata before the collection is published");

        match self.encrypted_metadata.pop() {
            Some(cyphertext) => {
                let plaintext = aes_gcm_decrypt(&password, &cyphertext);
                let data: HashMap<TokenId, TokenMetadata> = serde_json::from_str(&plaintext).unwrap();

                for (token_id, metadata) in data.iter() {
                    //insert the token ID and metadata
                    self.token_metadata_by_id.insert(&token_id, &metadata);
                }

                true
            },
            None => false
        }
    }
}