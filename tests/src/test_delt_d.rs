use std::collections::HashSet;

use crate::set_caller;
use delt_d::{
    character::CharacterManagment,
    staking::{StakeId, Staking, TokenType},
    Contract,
};
use near_sdk::{
    serde_json::{json, to_string},
    test_utils::{accounts, VMContextBuilder},
    testing_env, AccountId,
};

fn get_tokens(token_id: String, token_id_2: String) -> Vec<(AccountId, Vec<StakeId>)> {
    let token1 = StakeId {
        contract_id: accounts(1),
        token: TokenType::MT {
            token_id: token_id.clone(),
            balance: 1,
        },
    };

    let token2 = StakeId {
        contract_id: accounts(2),
        token: TokenType::FT { balance: 500 },
    };

    let token3 = StakeId {
        contract_id: accounts(1),
        token: TokenType::MT {
            token_id: token_id_2.clone(),
            balance: 1,
        },
    };

    let token4 = StakeId {
        contract_id: accounts(2),
        token: TokenType::FT { balance: 1000 },
    };

    let token5 = StakeId {
        contract_id: accounts(1),
        token: TokenType::FT { balance: 3000 },
    };

    let token6 = StakeId {
        contract_id: accounts(1),
        token: TokenType::FT { balance: 2000 },
    };

    vec![
        (accounts(3), vec![token1, token2]),
        (accounts(4), vec![token3, token4]),
        (accounts(5), vec![token5, token6]),
    ]
}

#[tokio::test]
async fn test_stake_pool() {
    let mut context = VMContextBuilder::new();

    testing_env!(context
        .current_account_id(accounts(0))
        .attached_deposit(10u128.pow(24))
        .signer_account_id(accounts(2))
        .build());

    let mut stake_contract = Contract::new(accounts(2));

    testing_env!(context.attached_deposit(10u128.pow(24)).build());

    stake_contract.set_default_attributes(
        to_string(
            json!(
                {
                    "attackSpeed": 3,
                    "hp": 100,
                    "hp_regen": 1,
                    "mp": 100,
                    "mp_regen": 1,
                    "speed": 200
                }
            )
            .as_object()
            .unwrap(),
        )
        .unwrap(),
    );

    stake_contract.register(None);

    for (player, _) in get_tokens("token1".to_string(), "token2".to_string()).into_iter() {
        stake_contract.register(Some(player));
    }

    let pool_id = "111".to_string();

    set_caller(&mut context, 2);

    testing_env!(context.attached_deposit(10u128.pow(24)).build());

    let mut pool_results = HashSet::new();

    pool_results.insert(accounts(3));
    pool_results.insert(accounts(4));

    stake_contract.create_pool(pool_id.clone(), pool_results, 0u128);

    println!("{:?}", stake_contract.get_pools(None).get(&pool_id));

    for (player, stakes) in get_tokens("token1".to_string(), "token2".to_string()).into_iter() {
        for stake in stakes.iter() {
            stake_contract.register_stake(stake.to_owned(), player.to_owned());
            stake_contract.stake(
                stake.to_owned(),
                player.to_owned(),
                player.to_owned(),
                pool_id.to_owned(),
                Vec::new(),
            );
        }
    }

    stake_contract.toggle_pool_active(pool_id.clone(), true);

    stake_contract.assert_pool_result(pool_id.clone(), Some(accounts(3)));

    stake_contract.distribute_stakes(pool_id.clone());
}
