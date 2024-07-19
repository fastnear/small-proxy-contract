mod proxy;

use crate::proxy::ProxyInput;
use near_sdk::store::LookupMap;
use near_sdk::{borsh, env, near, require, AccountId, BorshStorageKey, NearToken, PanicOnDefault};
use near_sdk::{Gas, GasWeight, Promise};

const PROXY_COST: NearToken = NearToken::from_millinear(20);
const PROXY_CONTRACT_BINARY: &[u8] = include_bytes!("../../small-proxy/res/small_proxy.wasm");

#[derive(BorshStorageKey)]
#[near]
enum StorageKeys {
    ProxyOwners,
}

#[derive(PanicOnDefault)]
#[near(contract_state)]
pub struct Contract {
    proxy_owners: LookupMap<AccountId, AccountId>,
}

#[near]
impl Contract {
    #[init]
    pub fn init() -> Self {
        Self {
            proxy_owners: LookupMap::new(StorageKeys::ProxyOwners),
        }
    }

    #[payable]
    pub fn deploy_proxy(&mut self, proxy_account_id: AccountId) -> Promise {
        require!(
            env::attached_deposit() >= PROXY_COST,
            "Not enough attached deposit"
        );
        let (_prefix, suffix) = proxy_account_id
            .as_str()
            .split_once('.')
            .expect("Invalid proxy account ID");
        require!(
            suffix == env::current_account_id().as_str(),
            "Invalid proxy account ID"
        );

        require!(
            self.proxy_owners
                .insert(proxy_account_id.clone(), env::predecessor_account_id())
                .is_none(),
            "Proxy already exists"
        );

        Promise::new(proxy_account_id)
            .create_account()
            .transfer(PROXY_COST)
            .deploy_contract(PROXY_CONTRACT_BINARY.to_vec())
            .function_call(
                "init".to_string(),
                vec![],
                NearToken::default(),
                Gas::from_tgas(1),
            )
    }

    pub fn call_proxy(&mut self, proxy_account_id: AccountId, input: ProxyInput) -> Promise {
        let predecessor_id = env::predecessor_account_id();
        let proxy_owner_id = self
            .proxy_owners
            .get(&proxy_account_id)
            .expect("Proxy account is not found");
        assert_eq!(
            &predecessor_id, proxy_owner_id,
            "Only proxy owner can call this method"
        );

        Promise::new(proxy_account_id).function_call_weight(
            "proxy".to_string(),
            borsh::to_vec(&input).expect("Failed to serialize input"),
            NearToken::default(),
            Gas::from_tgas(1),
            GasWeight::default(),
        )
    }
}
