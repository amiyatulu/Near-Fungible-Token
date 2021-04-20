/*!
* wNear NEP-141 Token contract
*
* The aim of the contract is to enable the wrapping of the native NEAR token into a NEP-141 compatible token.
* It supports methods `near_deposit` and `near_withdraw` that wraps and unwraps NEAR tokens.
* They are effectively mint and burn underlying wNEAR tokens.
*
* lib.rs is the main entry point.
* w_near.rs contains interfaces for depositing and withdrawing
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};

mod w_near;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub ft: FungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId,
        total_supply: U128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut this = Self {
            ft: FungibleToken::new(b"a".to_vec(), 5),
        };
        this.ft.internal_register_account(owner_id.as_ref());
        this.ft.internal_deposit(owner_id.as_ref(), total_supply.into());
        this
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);
near_contract_standards::impl_fungible_token_storage!(Contract, ft);

impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Wrapped NEAR fungible token"),
            symbol: String::from("wNEAR"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 24,
        }
    }
}
