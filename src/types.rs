use soroban_sdk::{contracttype, Address, BigInt};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TokenId,
    Open,
    Signups,
    Spots,
    Payments,
    Member(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct Payments {
    pub last_last: BigInt,
    pub last: BigInt,
    pub next: BigInt,
}

#[contracttype]
pub struct FaucetStatus {
    pub open: bool,
}