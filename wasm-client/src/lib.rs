use intmax_zkp_core::rollup::circuits::merge_and_purge::PurgeWitness;
use intmax_zkp_core::sparse_merkle_tree::gadgets::process::process_smt::SmtProcessProof;
use intmax_zkp_core::transaction::gadgets::merge::MergeProof;
use intmax_zkp_core::zkdsa::account::Address;
use intmax_zkp_core::zkdsa::circuits::prove_simple_signature as _prove_simple_signature;
use intmax_zkp_core::{
    rollup::circuits::merge_and_purge::prove_user_transaction as _prove_user_transaction,
    sparse_merkle_tree::goldilocks_poseidon::WrappedHashOut,
};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hash_types::HashOut;
use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

type F = GoldilocksField;

pub const N_LOG_MAX_USERS: usize = 8;
pub const N_LOG_MAX_TXS: usize = 8;
pub const N_LOG_MAX_CONTRACTS: usize = 8;
pub const N_LOG_MAX_VARIABLES: usize = 8;
pub const N_LOG_TXS: usize = 3;
pub const N_TXS: usize = 2usize.pow(N_LOG_TXS as u32);
pub const N_LOG_RECIPIENTS: usize = 8;
pub const N_LOG_CONTRACTS: usize = 8;
pub const N_LOG_VARIABLES: usize = 8;
pub const N_DEPOSITS: usize = 8;
pub const N_MERGES: usize = 8;
pub const N_DIFFS: usize = 8;
pub const N_BLOCKS: usize = 2;
pub const N_LOG_MAX_BLOCKS: usize = 32;

#[derive(Serialize, Deserialize)]
pub struct SimpleSignatureInput {
    private_key: WrappedHashOut<F>,
    message: WrappedHashOut<F>,
}

#[derive(Serialize, Deserialize)]
pub struct UserTransactionInput {
    sender_address: Address<F>,
    merge_witnesses: MergeProof<F>,
    purge_input_witnesses: PurgeWitness,
    purge_output_witnesses: PurgeWitness,
    old_user_asset_root: HashOut<F>,
}

#[wasm_bindgen(js_name = "proveSimpleSignature")]
pub fn prove_simple_signature(simple_signature_input_str: &str) -> String {
    let simple_signature_input: SimpleSignatureInput =
        serde_json::from_str(simple_signature_input_str)
            .expect("failed loading simple signature json");
    let proof_result = _prove_simple_signature::<
        N_LOG_MAX_USERS,
        N_LOG_MAX_TXS,
        N_LOG_MAX_CONTRACTS,
        N_LOG_MAX_VARIABLES,
        N_LOG_TXS,
        N_LOG_RECIPIENTS,
        N_LOG_CONTRACTS,
        N_LOG_VARIABLES,
        N_DIFFS,
        N_MERGES,
    >(
        simple_signature_input.private_key,
        simple_signature_input.message,
    )
    .unwrap();
    let proof_str = serde_json::to_string(&proof_result).unwrap();
    proof_str
}

#[wasm_bindgen(js_name = "proveUserTransaction")]
pub fn prove_user_transaction(user_transaction_input_str: &str) -> String {
    let user_transaction_input: UserTransactionInput =
        serde_json::from_str(user_transaction_input_str)
            .expect("failed loading user transaction json");
    let proof_result = _prove_user_transaction::<
        N_LOG_MAX_USERS,
        N_LOG_MAX_TXS,
        N_LOG_MAX_CONTRACTS,
        N_LOG_MAX_VARIABLES,
        N_LOG_TXS,
        N_LOG_RECIPIENTS,
        N_LOG_CONTRACTS,
        N_LOG_VARIABLES,
        N_DIFFS,
        N_MERGES,
    >(
        user_transaction_input.sender_address,
        &[user_transaction_input.merge_witnesses],
        user_transaction_input.purge_input_witnesses,
        user_transaction_input.purge_output_witnesses,
        user_transaction_input.old_user_asset_root,
    )
    .unwrap();
    let proof_str = serde_json::to_string(&proof_result).unwrap();
    proof_str
}
