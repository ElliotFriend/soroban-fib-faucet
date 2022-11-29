#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Accounts, Env};

#[test]
fn test_init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FibFaucetContract);
    let client = FibFaucetContractClient::new(&env, &contract_id);

    let admin_user = env.accounts().generate();

    client
        .with_source_account(&admin_user)
        .init(&10);
    // assert_eq!(client.get_admin(), client.contract_id())
}
