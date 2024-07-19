#![allow(internal_features)]
#![no_std]
#![feature(core_intrinsics)]
#![allow(non_snake_case)]

#[panic_handler]
#[no_mangle]
pub fn on_panic(_info: &::core::panic::PanicInfo) -> ! {
    ::core::intrinsics::abort();
}

#[allow(unused)]
extern "C" {
    // // #############
    // // # Registers #
    // // #############
    fn read_register(register_id: u64, ptr: u64);
    fn register_len(register_id: u64) -> u64;
    // // ###############
    // // # Context API #
    // // ###############
    // fn current_account_id(register_id: u64);
    // fn signer_account_id(register_id: u64);
    // fn signer_account_pk(register_id: u64);
    fn predecessor_account_id(register_id: u64);
    fn input(register_id: u64);
    // fn block_index() -> u64;
    // fn block_timestamp() -> u64;
    // fn epoch_height() -> u64;
    // fn storage_usage() -> u64;
    // // #################
    // // # Economics API #
    // // #################
    // fn account_balance(balance_ptr: u64);
    // fn account_locked_balance(balance_ptr: u64);
    // fn attached_deposit(balance_ptr: u64);
    // fn prepaid_gas() -> u64;
    // fn used_gas() -> u64;
    // // ############
    // // # Math API #
    // // ############
    // fn random_seed(register_id: u64);
    fn sha256(value_len: u64, value_ptr: u64, register_id: u64);
    // fn keccak256(value_len: u64, value_ptr: u64, register_id: u64);
    // fn keccak512(value_len: u64, value_ptr: u64, register_id: u64);
    // // #####################
    // // # Miscellaneous API #
    // // #####################
    // fn value_return(value_len: u64, value_ptr: u64);
    fn panic();
    fn panic_utf8(len: u64, ptr: u64);
    // fn log_utf8(len: u64, ptr: u64);
    // fn log_utf16(len: u64, ptr: u64);
    // // ################
    // // # Promises API #
    // // ################
    // fn promise_create(
    //     account_id_len: u64,
    //     account_id_ptr: u64,
    //     method_name_len: u64,
    //     method_name_ptr: u64,
    //     arguments_len: u64,
    //     arguments_ptr: u64,
    //     amount_ptr: u64,
    //     gas: u64,
    // ) -> u64;
    // fn promise_then(
    //     promise_index: u64,
    //     account_id_len: u64,
    //     account_id_ptr: u64,
    //     method_name_len: u64,
    //     method_name_ptr: u64,
    //     arguments_len: u64,
    //     arguments_ptr: u64,
    //     amount_ptr: u64,
    //     gas: u64,
    // ) -> u64;
    // fn promise_and(promise_idx_ptr: u64, promise_idx_count: u64) -> u64;
    fn promise_batch_create(account_id_len: u64, account_id_ptr: u64) -> u64;
    fn promise_batch_then(promise_index: u64, account_id_len: u64, account_id_ptr: u64) -> u64;
    // // #######################
    // // # Promise API actions #
    // // #######################
    fn promise_batch_action_create_account(promise_index: u64);
    fn promise_batch_action_deploy_contract(promise_index: u64, code_len: u64, code_ptr: u64);
    fn promise_batch_action_function_call(
        promise_index: u64,
        method_name_len: u64,
        method_name_ptr: u64,
        arguments_len: u64,
        arguments_ptr: u64,
        amount_ptr: u64,
        gas: u64,
    );
    fn promise_batch_action_transfer(promise_index: u64, amount_ptr: u64);
    fn promise_batch_action_stake(
        promise_index: u64,
        amount_ptr: u64,
        public_key_len: u64,
        public_key_ptr: u64,
    );
    fn promise_batch_action_add_key_with_full_access(
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
        nonce: u64,
    );
    fn promise_batch_action_add_key_with_function_call(
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
        nonce: u64,
        allowance_ptr: u64,
        receiver_id_len: u64,
        receiver_id_ptr: u64,
        method_names_len: u64,
        method_names_ptr: u64,
    );
    fn promise_batch_action_delete_key(
        promise_index: u64,
        public_key_len: u64,
        public_key_ptr: u64,
    );
    fn promise_batch_action_delete_account(
        promise_index: u64,
        beneficiary_id_len: u64,
        beneficiary_id_ptr: u64,
    );
    // // #######################
    // // # Promise API results #
    // // #######################
    // fn promise_results_count() -> u64;
    // fn promise_result( result_idx: u64, register_id: u64) -> u64;
    // fn promise_return( promise_id: u64);
    // ###############
    // # Storage API #
    // ###############
    fn storage_write(
        key_len: u64,
        key_ptr: u64,
        value_len: u64,
        value_ptr: u64,
        register_id: u64,
    ) -> u64;
    fn storage_read(key_len: u64, key_ptr: u64, register_id: u64) -> u64;
    // fn storage_remove( key_len: u64, key_ptr: u64, register_id: u64) -> u64;
    fn storage_has_key(key_len: u64, key_ptr: u64) -> u64;
    // // ###############
    // // # Validator API #
    // // ###############
    // fn validator_stake( account_id_len: u64, account_id_ptr: u64, stake_ptr: u64);
    // fn validator_total_stake( stake_ptr: u64);
}

const HASH_LEN: u64 = 32;
const FROM_REGISTER: u64 = u64::MAX;

#[derive(Default, Clone, PartialEq)]
struct HashedAccountId(pub [u8; HASH_LEN as _]);

#[repr(u8)]
enum StorageKeys {
    OwnerId = 0,
}

#[allow(dead_code)]
#[repr(u8)]
enum ActionType {
    CreateAccount = 0,
    DeployContract,
    FunctionCall,
    Transfer,
    Stake,
    AddFullAccessKey,
    AddFunctionCallKey,
    DeleteKey,
    DeleteAccount,
}

#[no_mangle]
pub unsafe fn init() {
    if storage_has_key(1, StorageKeys::OwnerId as _) == 1 {
        panic();
    }
    predecessor_account_id(0);
    sha256(FROM_REGISTER, 0, 0);
    storage_write(1, StorageKeys::OwnerId as _, FROM_REGISTER, 0, 0);
}

#[no_mangle]
pub unsafe fn proxy() {
    // Verify predecessor
    assert_owner();

    // Read input
    input(0);
    let input_len = register_len(0);
    let mut input_ptr = ensure_memory(input_len as _) as u64;
    read_register(0, input_ptr);

    let num_promises = read_u32(&mut input_ptr);
    let mut promise_index = 0;
    for i in 0..num_promises {
        let account_id_len = read_u32(&mut input_ptr);
        promise_index = if i == 0 {
            promise_batch_create(account_id_len as _, input_ptr as _)
        } else {
            promise_batch_then(promise_index, account_id_len as _, input_ptr as _)
        };
        input_ptr += account_id_len as u64;

        let num_actions = read_u32(&mut input_ptr);
        for _ in 0..num_actions {
            let action_type: u8 = read_u8(&mut input_ptr);
            let action_type = core::mem::transmute(action_type);
            match action_type {
                ActionType::CreateAccount => {
                    promise_batch_action_create_account(promise_index);
                }
                ActionType::DeployContract => {
                    let (code_len, code_ptr) = read_buf(&mut input_ptr);
                    promise_batch_action_deploy_contract(promise_index, code_len, code_ptr);
                }
                ActionType::FunctionCall => {
                    let (method_name_len, method_name_ptr) = read_buf(&mut input_ptr);
                    let (arguments_len, arguments_ptr) = read_buf(&mut input_ptr);
                    let amount_ptr = read_u128_ptr(&mut input_ptr);
                    let gas = read_u64(&mut input_ptr);
                    promise_batch_action_function_call(
                        promise_index,
                        method_name_len,
                        method_name_ptr,
                        arguments_len,
                        arguments_ptr,
                        amount_ptr,
                        gas,
                    );
                }
                ActionType::Transfer => {
                    let amount_ptr = read_u128_ptr(&mut input_ptr);
                    promise_batch_action_transfer(promise_index, amount_ptr);
                }
                ActionType::Stake => {
                    let amount_ptr = read_u128_ptr(&mut input_ptr);
                    let (public_key_len, public_key_ptr) = read_public_key(&mut input_ptr);
                    promise_batch_action_stake(
                        promise_index,
                        amount_ptr,
                        public_key_len,
                        public_key_ptr,
                    );
                }
                ActionType::AddFullAccessKey => {
                    let (public_key_len, public_key_ptr) = read_public_key(&mut input_ptr);
                    let nonce = read_u64(&mut input_ptr);
                    promise_batch_action_add_key_with_full_access(
                        promise_index,
                        public_key_len,
                        public_key_ptr,
                        nonce,
                    );
                }
                ActionType::AddFunctionCallKey => {
                    let (public_key_len, public_key_ptr) = read_public_key(&mut input_ptr);
                    let nonce = read_u64(&mut input_ptr);
                    let allowance_ptr = read_u128_ptr(&mut input_ptr);
                    let (receiver_id_len, receiver_id_ptr) = read_buf(&mut input_ptr);
                    let (method_names_len, method_names_ptr) = read_buf(&mut input_ptr);
                    promise_batch_action_add_key_with_function_call(
                        promise_index,
                        public_key_len,
                        public_key_ptr,
                        nonce,
                        allowance_ptr,
                        receiver_id_len,
                        receiver_id_ptr,
                        method_names_len,
                        method_names_ptr,
                    );
                }
                ActionType::DeleteKey => {
                    let (public_key_len, public_key_ptr) = read_public_key(&mut input_ptr);
                    promise_batch_action_delete_key(promise_index, public_key_len, public_key_ptr);
                }
                ActionType::DeleteAccount => {
                    let (beneficiary_id_len, beneficiary_id_ptr) = read_buf(&mut input_ptr);
                    promise_batch_action_delete_account(
                        promise_index,
                        beneficiary_id_len,
                        beneficiary_id_ptr,
                    );
                }
            }
        }
    }
}

//
// HELPERS
//

unsafe fn assert_owner() {
    let mut owner_id = HashedAccountId::default();
    storage_read(1, StorageKeys::OwnerId as _, 0);
    read_register(0, owner_id.0.as_mut_ptr() as _);
    let mut predecessor_id = HashedAccountId::default();
    predecessor_account_id(0);
    sha256(FROM_REGISTER, 0, 0);
    read_register(0, predecessor_id.0.as_mut_ptr() as _);
    if owner_id != predecessor_id {
        panic();
    }
}

fn ensure_memory(size: usize) -> *mut u8 {
    let pages_needed = (size + 65535) / 65536;
    let current_pages = core::arch::wasm32::memory_size(0);
    let _ = core::arch::wasm32::memory_grow(0, pages_needed);

    (current_pages * 65536) as *mut u8
}

unsafe fn read_u32(ptr: &mut u64) -> u32 {
    let mut buf = [0u8; 4];
    buf.copy_from_slice(core::slice::from_raw_parts(*ptr as _, 4));
    *ptr += 4;
    u32::from_le_bytes(buf)
}

unsafe fn read_u64(ptr: &mut u64) -> u64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(core::slice::from_raw_parts(*ptr as _, 8));
    *ptr += 8;
    u64::from_le_bytes(buf)
}

#[inline]
unsafe fn read_u8(ptr: &mut u64) -> u8 {
    let res = *(*ptr as *const u8);
    *ptr += 1;
    res
}

unsafe fn read_u128_ptr(ptr: &mut u64) -> u64 {
    let res = *ptr;
    *ptr += 16;
    res
}

/// Returns a pair of (len, ptr) pointing to the beginning of the buffer data.
unsafe fn read_buf(ptr: &mut u64) -> (u64, u64) {
    let len = read_u32(ptr) as u64;
    let res_ptr = *ptr;
    *ptr += len;
    (len, res_ptr)
}

/// Returns a pair of (len, ptr) pointing to the public key according to borsh serialization.
unsafe fn read_public_key(ptr: &mut u64) -> (u64, u64) {
    let res_ptr = *ptr;
    let pk_type = *(*ptr as *const u8);
    let len: u64 = if pk_type == 0 {
        33 // ed25519
    } else {
        65 // sec256k1
    };
    *ptr += len;
    (len, res_ptr)
}
