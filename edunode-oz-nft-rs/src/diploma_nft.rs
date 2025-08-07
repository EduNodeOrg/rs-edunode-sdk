use soroban_sdk::{contract, contractimpl, Env, Address, String, Bytes};
use stellar_default_impl_macro::default_impl;
use stellar_non_fungible::{
    Base,
    enumerable::{Enumerable, NonFungibleEnumerable},
    NonFungibleToken,
};
use stellar_ownable::{self as ownable, Ownable};
use stellar_ownable_macro::only_owner;
use stellar_pausable::{self as pausable, Pausable};
use stellar_pausable_macros::when_not_paused;
use stellar_upgradeable::{UpgradeableInternal};
use stellar_upgradeable_macros::Upgradeable;

const COURSE_ID_KEY: &str = "course_id";

#[derive(Upgradeable)]
#[contract]
pub struct DiplomaNFT;

#[contractimpl]
impl DiplomaNFT {
    // Initialize contract with owner and metadata
    pub fn __constructor(
        e: &Env, 
        owner: Address,
        course_id: String,
        uri: String,
        name: String,
        symbol: String
    ) {
        Base::set_metadata(e, uri, name, symbol);
        ownable::set_owner(e, &owner);
        
        // Store the course ID
        e.storage().persistent().set(&Bytes::from_slice(e, COURSE_ID_KEY.as_bytes()), &course_id);
    }

    // Get the course ID for this contract
    pub fn get_course_id(e: &Env) -> String {
        e.storage().persistent().get(&Bytes::from_slice(e, COURSE_ID_KEY.as_bytes()))
            .unwrap_or_else(|| panic!("Course ID not set"))
    }

    // Only the owner (issuer) can mint a diploma
    #[only_owner]
    #[when_not_paused]
    pub fn mint(e: &Env, to: Address) -> u32 {
        Enumerable::sequential_mint(e, &to)
    }
}

// Trait required for standard NFT structure
#[default_impl]
#[contractimpl]
impl NonFungibleToken for DiplomaNFT {
    type ContractType = Enumerable;

    // Disable transfers (diplomas are non-transferable)
    #[when_not_paused]
    fn transfer(_e: &Env, _from: Address, _to: Address, _token_id: u32) {
        panic!("Transfer not allowed: Diplomas are non-transferable");
    }

    #[when_not_paused]
    fn transfer_from(_e: &Env, _spender: Address, _from: Address, _to: Address, _token_id: u32) {
        panic!("Transfer not allowed: Diplomas are non-transferable");
    }
}

// Enable enumerable tracking of diplomas
#[default_impl]
#[contractimpl]
impl NonFungibleEnumerable for DiplomaNFT {}

//
// Upgradeable logic restricted to owner
//
impl UpgradeableInternal for DiplomaNFT {
    fn _require_auth(e: &Env, _operator: &Address) {
        ownable::enforce_owner_auth(e);
    }
}

//
// Pausable: Owner can pause/unpause minting
//
#[contractimpl]
impl Pausable for DiplomaNFT {
    fn paused(e: &Env) -> bool {
        pausable::paused(e)
    }

    #[only_owner]
    fn pause(e: &Env, _caller: Address) {
        pausable::pause(e);
    }

    #[only_owner]
    fn unpause(e: &Env, _caller: Address) {
        pausable::unpause(e);
    }
}

// Ownable implementation
#[default_impl]
#[contractimpl]
impl Ownable for DiplomaNFT {}