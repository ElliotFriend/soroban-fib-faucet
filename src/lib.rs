#![no_std]

use soroban_auth::{
    {Identifier, Signature}
};

use soroban_sdk::{
    contractimpl, bigint, bytes, vec, panic_with_error,
    AccountId, Address, BigInt, Bytes, BytesN, Env, Vec
};

mod token {
    soroban_sdk::contractimport!(file = "./soroban_token_spec.wasm");
}

use error::Error;
use types::*;

// FIB Smart Token

fn init_token(env: &Env) {
    let mut salt = Bytes::new(&env);
    salt.append(&bytes!(&env, [0,1,1,2,3,5,8,13,21,34,55]));
    let salt = env.compute_hash_sha256(&salt);
    let token_id = env.deployer().with_current_contract(salt).deploy_token();

    let tok_metadata = token::TokenMetadata {
        name: bytes!(&env, 0x4669626F6E6163636920466175636574),
        symbol: bytes!(&env, 0x464942),
        decimals: 0,
    };
    token::Client::new(&env, &token_id).init(
        &Identifier::Contract(env.current_contract()),
        &tok_metadata,
    );

    put_token_id(&env, token_id);
}

fn put_token_id(env: &Env, token_id: BytesN<32>) {
    let key = DataKey::TokenId;
    env.data().set(key, token_id)
}

fn get_token_id(env: &Env) -> BytesN<32> {
    let key = DataKey::TokenId;
    env.data()
        .get(key)
        .unwrap_or_else(|| panic_with_error!(env, Error::FaucetNotInit))
        .unwrap()
}

// Faucet Admin

fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;
    env.data().has(key)
}

fn get_admin(env: &Env) -> Address {
    let key = DataKey::Admin;
    env.data()
        .get(key)
        .unwrap_or_else(|| panic_with_error!(env, Error::NoAdmin))
        .unwrap()
}

fn put_admin(env: &Env, admin: Address) {
    let key = DataKey::Admin;
    env.data().set(key, admin);
}

fn check_admin(env: &Env, address: Address) -> bool {
    address == get_admin(&env)
}

fn is_open(env: &Env) -> bool {
    let key = DataKey::Open;
    env.data()
        .get(key)
        .unwrap_or(Ok(false))
        .unwrap()
}

fn put_open(env: &Env, open: bool) {
    let key = DataKey::Open;
    env.data().set(key, open);
}

fn put_spots(env: &Env, spots: u32) {
    let key = DataKey::Spots;
    env.data().set(key, spots);
}

fn get_spots(env: &Env) -> u32 {
    let key = DataKey::Spots;
    env.data()
        .get(key)
        .unwrap_or(Ok(10))
        .unwrap()
}

fn spots_available(env: &Env) -> bool {
    let spots = get_spots(&env);
    let signups = get_signups(&env);
    spots > signups.len()
}

// User Management

fn get_account_id(env: &Env, invoker: Address) -> AccountId {
    match invoker {
        Address::Account(account_id) => account_id,
        Address::Contract(_) => panic_with_error!(&env, Error::NoCrossContract)
    }
}

fn is_not_contract(env: &Env) -> bool {
    match env.invoker() {
        Address::Account(_) => true,
        Address::Contract(_) => false
    }
}

fn is_member(env: &Env, member: &Address) -> bool {
    let key = DataKey::Member(member.clone());
    env.data().has(key)
}

fn put_member(env: &Env, member: Address, amount: BigInt) {
    let key = DataKey::Member(member);
    env.data().set(key, amount);
}

fn get_member(env: &Env, member: &Address) -> BigInt {
    let key = DataKey::Member(member.clone());
    env.data()
        .get(key)
        .unwrap()
        .unwrap()
}

fn put_signup(env: &Env, member: Address) {
    let key = DataKey::Signups;
    let mut signups = get_signups(&env);
    signups.push_back(member);
    env.data().set(key, signups);
}

fn get_signups(env: &Env) -> Vec<Address> {
    let key = DataKey::Signups;
    env.data()
        .get(key)
        .unwrap_or(Ok(vec![&env]))
        .unwrap()
}

fn empty_signups(env: &Env) {
    let key = DataKey::Signups;
    env.data().set(key, Vec::<Address>::new(&env));
}

// Payments & Numbers

fn get_payments(env: &Env) -> Payments {
    let key = DataKey::Payments;
    env.data()
        .get(key)
        .unwrap_or_else(|| Ok(
            Payments { last_last: BigInt::zero(env), last: BigInt::zero(env), next: BigInt::zero(env) }
        ))
        .unwrap()
}

fn put_payments(env: &Env, payments: Payments) {
    let key = DataKey::Payments;
    env.data().set(key, payments)
}

fn calc_payments(env: &Env, payments: Payments) -> Payments {
    if payments.next == 0 {
        Payments {
            last_last: bigint!(&env, 0),
            last: bigint!(&env, 0),
            next: bigint!(&env, 1),
        }
    } else {
        let mut next_payment = &payments.last + &payments.next;
        if next_payment == 0 {
            next_payment = bigint!(&env, 1);
        }
        Payments {
            last_last: payments.last,
            last: payments.next,
            next: next_payment,
        }
    }
}

fn mint_fib_token(env: &Env, to: &Address, amount: BigInt) {
    let token_id = get_token_id(&env);
    token::Client::new(&env, token_id).mint(
        &Signature::Invoker,
        &BigInt::zero(&env),
        &Identifier::from(to),
        &amount,
    );
}

pub trait FibFaucetTrait {
    fn init(env: Env, spots: u32) -> Result<(), Error>;
    fn signup(env: Env) -> Result<(), Error>;
    fn curr_pay(env: Env) -> Payments;
    fn open(env: Env) -> Result<(), Error>;
    fn close(env: Env) -> Result<(), Error>;
    fn disburse(env: Env) -> Result<(), Error>;
    fn reset(env: Env, spots: u32, open: bool) -> Result<(), Error>;
    fn member(env: Env, member: Address) -> Result<BigInt, Error>;
    fn signups(env: Env) -> Vec<Address>;
    fn status(env: Env) -> FaucetStatus;
    fn admin(env: Env) -> Result<Address, Error>;
}

pub struct FibFaucetContract;

#[contractimpl]
impl FibFaucetTrait for FibFaucetContract {

    fn init(env: Env, spots: u32) -> Result<(), Error> {
        if !has_admin(&env) {
            if is_not_contract(&env) {
                let admin_id = get_account_id(&env, env.invoker());
                put_admin(&env, Address::Account(admin_id));

                init_token(&env);
                put_open(&env, true);
                put_spots(&env, spots);
                empty_signups(&env);
                put_payments(&env, get_payments(&env));
                Ok(())
            } else {
                Err(Error::NoCrossContract)
            }
        } else {
            Err(Error::FaucetAlreadyInit)
        }
    }

    fn signup(env: Env) -> Result<(), Error> {
        if is_not_contract(&env)
        && !is_member(&env, &env.invoker())
        && !check_admin(&env, env.invoker()) {
            if is_open(&env) && spots_available(&env) {
                let payments = get_payments(&env);
                put_signup(&env, env.invoker());
                put_member(&env, env.invoker(), BigInt::from(payments.next.clone()));

                let new_payments = calc_payments(&env, payments);
                put_payments(&env, new_payments);

                Ok(())
            } else if !spots_available(&env) {
                Err(Error::SignupsFull)
            } else {
                Err(Error::FaucetClosed)
            }
        } else {
            Err(Error::InvalidSignup)
        }
    }

    fn curr_pay(env: Env) -> Payments {
        get_payments(&env)
    }

    fn open(env: Env) -> Result<(), Error> {
        if check_admin(&env, env.invoker()) {
            put_open(&env, true);
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    fn close(env: Env) -> Result<(), Error> {
        if check_admin(&env, env.invoker()) {
            put_open(&env, false);
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    fn member(env: Env, member: Address) -> Result<BigInt, Error> {
        if is_member(&env, &member) {
            Ok(get_member(&env, &member))
        } else {
            Err(Error::NoSuchMember)
        }
    }

    fn signups(env: Env) -> Vec<Address> {
        get_signups(&env)
    }

    fn reset(env: Env, spots: u32, open: bool) -> Result<(), Error> {
        if check_admin(&env, env.invoker()) {
            let signups = get_signups(&env);
            if signups.len() == 0 {
                put_spots(&env, spots);
                put_open(&env, open);
                put_payments(&env, Payments {
                    last_last: BigInt::zero(&env), last: BigInt::zero(&env), next: BigInt::zero(&env)
                });
                Ok(())
            } else {
                Err(Error::DisburseRequired)
            }
        } else {
            Err(Error::Unauthorized)
        }
    }

    fn disburse(env: Env) -> Result<(), Error> {
        if check_admin(&env, env.invoker()) {
            let signups = get_signups(&env);
            if signups.len() > 0 {
                for member in signups.iter() {
                    let memb = &member.unwrap();
                    let amount = get_member(&env, &memb);
                    mint_fib_token(&env, &memb, amount);
                }
                empty_signups(&env);
                Ok(())
            } else {
                Err(Error::NoSignups)
            }
        } else {
            Err(Error::Unauthorized)
        }
    }

    fn status(env: Env) -> FaucetStatus {
        FaucetStatus { open: is_open(&env) }
    }

    fn admin(env: Env) -> Result<Address, Error> {
        if has_admin(&env) {
            Ok(get_admin(&env))
        } else {
            Err(Error::NoAdmin)
        }
    }
}

mod error;
mod test;
mod types;