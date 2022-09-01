use crate::*;
use near_sdk::env;

impl Contract {
    pub(crate) fn assert_called_by_owner(&self) {
        let sender_id = env::predecessor_account_id();

        //make sure the sender ID is the contract owner. 
        assert_eq!(
            self.owner_id,
            sender_id,
            "owner_id should be sender_id"
        );
    }
} 