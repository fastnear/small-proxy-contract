use crate::*;
use near_sdk::json_types::Base64VecU8;
use near_sdk::{Gas, NearToken, PublicKey};

#[near(serializers=[borsh, json])]
pub struct ProxyInput(pub Vec<ProxyPromise>);

#[near(serializers=[borsh, json])]
pub struct ProxyPromise {
    pub account_id: AccountId,
    pub actions: Vec<ProxyAction>,
}

#[near(serializers=[borsh, json])]
pub enum ProxyAction {
    CreateAccount,
    DeployContract(ProxyActionDeployContract),
    FunctionCall(ProxyActionFunctionCall),
    Transfer(ProxyActionTransfer),
    Stake(ProxyActionStake),
    AddFullAccessKey(ProxyActionAddFullAccessKey),
    AddFunctionCallKey(ProxyActionAddFunctionCallKey),
    DeleteKey(ProxyActionDeleteKey),
    DeleteAccount(ProxyActionDeleteAccount),
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionDeployContract {
    pub contract_code: Base64VecU8,
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionFunctionCall {
    pub method_name: String,
    pub args: Base64VecU8,
    pub deposit: NearToken,
    pub gas: Gas,
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionTransfer {
    pub amount: NearToken,
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionStake {
    pub amount: NearToken,
    pub public_key: PublicKey,
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionAddFullAccessKey {
    pub public_key: PublicKey,
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionAddFunctionCallKey {
    pub public_key: PublicKey,
    pub nonce: u64,
    pub allowance: NearToken,
    pub receiver_id: AccountId,
    // Comma-separated list of method names. Empty means all methods.
    pub method_names: String,
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionDeleteKey {
    pub public_key: PublicKey,
}

#[near(serializers=[borsh, json])]
pub struct ProxyActionDeleteAccount {
    pub beneficiary_id: AccountId,
}
