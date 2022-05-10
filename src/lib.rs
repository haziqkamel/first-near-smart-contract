/// Writing the contract
///
/// Create a simple CRUD backend in Rust that utilizes
/// the on-chain storage offered by NEAR.
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen};

// near_sdk::setup_alloc!();

// Setup the global allocator from the [wee_alloc] crate using the [setup_alloc!()] macro.
// Allocators are the way that programs in Rust obtain memory from the system at runtime. [wee_alloc] is a
// memory allocator designed for WebAssembly. It generates less than a kilobyte of uncompressed WebAssyembly code.

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

// Follow a patter using one structure (struct) and an implementation (impl)
// associated with it. This is a pattern used in most Rust contracts on NEAR.

// 1. Main Struct
//
// Attributes is a declarative tag that is used to convey information to runtime about the behaviors
// of various elements like classes, methods, structures, enumerators, assemblies, etc.
//
// By adding the macro #[near_bindgen] we provide our struct with generated boilerplate code to make it
// compatible with the NEAR blockchain.
//
// The second macro, #[derive(BorshDeserialize, BorshSerialize)] assists in the serialization and de-serialization
// of the data for sending it to or retrieving it from NEAR

// Attributes of struct KeyValue
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KeyValue {
    pairs: UnorderedMap<String, String>,
}

// 2. Default Implementation
//
// Default implementation for KeyValue struct
impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pairs: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

// 3. Core Logic
//
// Add methods to KeyValue struct
#[near_bindgen]
impl KeyValue {
    pub fn create_update(&mut self, k: String, v: String) {
        env::log(b"created or update"); // log fn from near-sdk
        self.pairs.insert(&k, &v); // insert into UnorderedMap
    }

    pub fn read(&self, k: String) -> Option<String> {
        env::log(b"read");
        return self.pairs.get(&k); //get value from pairs from key: &k
    }

    pub fn delete(&mut self, k: String) {
        env::log(b"delete");
        self.pairs.remove(&k); // remove from pairs key: &k
    }
}

// 4. Tests
//
// Writing unit test are imprtant because smart contracts are often
// immutable and sometimes responsible for managing sums of money.
//
// Writing good unit tests is a key component of secure and reliable
// smart contract development.
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    // Test 1
    //
    /// Test for [create_update] and [read]
    #[test]
    fn create_read_pair() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = KeyValue::default();
        contract.create_update("first_key".to_string(), "hello".to_string());

        assert_eq!(
            "hello".to_string(),
            contract.read("first_key".to_string()).unwrap()
        ); // expect("hello", "hello")
    }

    // Test 2
    //
    // Test for read non-existent pair
    #[test]
    fn read_nonexistent_pair() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = KeyValue::default();
        assert_eq!(None, contract.read("first_key".to_string()));
    }
}
