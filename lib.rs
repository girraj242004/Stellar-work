#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, log};

#[contracttype]
pub enum DataKey {
    Verifier(Address),
    UserKYCStatus(Address),
}

#[contract]
pub struct KycVerifier;

#[contractimpl]
impl KycVerifier {
    // Add an approved KYC verifier
    pub fn add_verifier(env: Env, admin: Address, verifier: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Verifier(verifier.clone()), &true);
        log!(&env, "Verifier added: {:?}", verifier);
    }

    // Submit user KYC status by a verifier
    pub fn verify_user(env: Env, verifier: Address, user: Address, status: bool) {
        verifier.require_auth();

        let is_verifier = env.storage().instance().get(&DataKey::Verifier(verifier.clone())).unwrap_or(false);
        if !is_verifier {
            panic!("Not an authorized verifier");
        }

        env.storage().instance().set(&DataKey::UserKYCStatus(user.clone()), &status);
        log!(&env, "User {:?} verified: {}", user, status);
    }

    // Check KYC status of a user
    pub fn is_verified(env: Env, user: Address) -> bool {
        env.storage().instance().get(&DataKey::UserKYCStatus(user)).unwrap_or(false)
    }
}
