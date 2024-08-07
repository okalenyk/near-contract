use near_sdk::{env, near, AccountId, PanicOnDefault};

type Version = [u8; 2];

pub type TokenId = String;

#[near(serializers = [json])]
pub struct Token {
    pub token_id: TokenId,
    pub owner_id: AccountId,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Account {
    pub token_contract: AccountId,

    pub token_id: TokenId,

    pub state: u128,

    pub version: Version,
}

#[near]
impl Account {
    #[init]
    pub fn new(version: Version, token_id: TokenId, token_contract: AccountId) -> Self {
        let this = Self {
            token_id: token_id.clone(),
            token_contract: token_contract.clone(),
            state: 0,
            version,
        };

        this
    }

    /// Contract metadata and methods in the API may be updated. All other
    /// elements of the state should be copied over. This method may only be
    /// called by the holder of the Store public key, in this case the
    /// Factory.
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let this: Account =
            env::state_read().unwrap_or_else(|| env::panic_str("ERR_CONTRACT_IS_NOT_INITIALIZED"));

        this
    }
}

#[cfg(test)]
mod tests {}
