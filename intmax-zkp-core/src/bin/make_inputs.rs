use std::{
    fs,
    sync::{Arc, Mutex},
    time::Instant,
};

use plonky2::{
    field::{
        goldilocks_field::GoldilocksField,
        types::{Field, Field64, Sample},
    },
    hash::hash_types::HashOut,
    iop::witness::PartialWitness,
    plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
};

use intmax_zkp_core::{
    rollup::circuits::merge_and_purge::{make_user_proof_circuit, PurgeWitness},
    sparse_merkle_tree::{
        goldilocks_poseidon::{
            GoldilocksHashOut, LayeredLayeredPoseidonSparseMerkleTree, NodeDataMemory,
            PoseidonSparseMerkleTree, WrappedHashOut, Wrapper,
        },
        proof::SparseMerkleInclusionProof,
    },
    transaction::{block_header::BlockHeader, gadgets::merge::MergeProof},
    zkdsa::{
        account::{private_key_to_account, Address},
        circuits::make_simple_signature_circuit,
    },
};
use serde::{Deserialize, Serialize};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type H = <C as GenericConfig<D>>::InnerHasher;
type F = <C as GenericConfig<D>>::F;

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

fn main() {
    const N_LOG_MAX_USERS: usize = 3;
    const N_LOG_MAX_TXS: usize = 3;
    const N_LOG_MAX_CONTRACTS: usize = 3;
    const N_LOG_MAX_VARIABLES: usize = 3;
    const N_LOG_TXS: usize = 2;
    const N_LOG_RECIPIENTS: usize = 3;
    const N_LOG_CONTRACTS: usize = 3;
    const N_LOG_VARIABLES: usize = 3;
    const N_DEPOSITS: usize = 2;
    const N_DIFFS: usize = 2;
    const N_MERGES: usize = 2;
    const N_TXS: usize = 2usize.pow(N_LOG_TXS as u32);
    const N_BLOCKS: usize = 2;

    let mut world_state_tree = PoseidonSparseMerkleTree::new(
        Arc::new(Mutex::new(NodeDataMemory::default())),
        Default::default(),
    );

    let merge_and_purge_circuit = make_user_proof_circuit::<
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
    >();

    // dbg!(&purge_proof_circuit_data.common);

    let sender1_private_key = HashOut {
        elements: [
            GoldilocksField::from_canonical_u64(17426287337377512978),
            GoldilocksField::from_canonical_u64(8703645504073070742),
            GoldilocksField::from_canonical_u64(11984317793392655464),
            GoldilocksField::from_canonical_u64(9979414176933652180),
        ],
    };
    let sender1_account = private_key_to_account(sender1_private_key);
    let sender1_address = sender1_account.address.0;

    let mut sender1_user_asset_tree: LayeredLayeredPoseidonSparseMerkleTree<NodeDataMemory> =
        LayeredLayeredPoseidonSparseMerkleTree::new(Default::default(), Default::default());

    let mut sender1_tx_diff_tree: LayeredLayeredPoseidonSparseMerkleTree<NodeDataMemory> =
        LayeredLayeredPoseidonSparseMerkleTree::new(Default::default(), Default::default());

    let key1 = (
        GoldilocksHashOut::from_u128(12),
        GoldilocksHashOut::from_u128(305),
        GoldilocksHashOut::from_u128(8012),
    );
    let value1 = GoldilocksHashOut::from_u128(2053);
    let key2 = (
        GoldilocksHashOut::from_u128(12),
        GoldilocksHashOut::from_u128(471),
        GoldilocksHashOut::from_u128(8012),
    );
    let value2 = GoldilocksHashOut::from_u128(1111);

    let key3 = (
        GoldilocksHashOut::from_u128(407),
        GoldilocksHashOut::from_u128(305),
        GoldilocksHashOut::from_u128(8012),
    );
    let value3 = GoldilocksHashOut::from_u128(2053);
    let key4 = (
        GoldilocksHashOut::from_u128(832),
        GoldilocksHashOut::from_u128(471),
        GoldilocksHashOut::from_u128(8012),
    );
    let value4 = GoldilocksHashOut::from_u128(1111);

    let zero = GoldilocksHashOut::from_u128(0);
    sender1_user_asset_tree
        .set(key1.0, key1.1, key1.2, value1)
        .unwrap();
    sender1_user_asset_tree
        .set(key2.0, key2.1, key2.2, value2)
        .unwrap();

    world_state_tree
        .set(
            sender1_account.address.0.into(),
            sender1_user_asset_tree.get_root(),
        )
        .unwrap();

    let proof1 = sender1_user_asset_tree
        .set(key2.0, key2.1, key2.2, zero)
        .unwrap();
    let proof2 = sender1_user_asset_tree
        .set(key1.0, key1.1, key1.2, zero)
        .unwrap();

    let proof3 = sender1_tx_diff_tree
        .set(key3.0, key3.1, key3.2, value3)
        .unwrap();
    let proof4 = sender1_tx_diff_tree
        .set(key4.0, key4.1, key4.2, value4)
        .unwrap();

    let sender1_input_witness = vec![proof1, proof2];
    let sender1_output_witness = vec![proof3, proof4];

    let sender2_private_key: HashOut<F> = HashOut::rand();
    dbg!(&sender2_private_key);
    let sender2_account = private_key_to_account(sender2_private_key);
    let sender2_address = sender2_account.address.0;

    let node_data = Arc::new(Mutex::new(NodeDataMemory::default()));
    let mut sender2_user_asset_tree =
        PoseidonSparseMerkleTree::new(node_data.clone(), Default::default());

    let mut sender2_tx_diff_tree =
        LayeredLayeredPoseidonSparseMerkleTree::new(node_data.clone(), Default::default());

    let mut deposit_sender2_tree =
        LayeredLayeredPoseidonSparseMerkleTree::new(node_data.clone(), Default::default());

    deposit_sender2_tree
        .set(sender2_address.into(), key1.1, key1.2, value1)
        .unwrap();
    deposit_sender2_tree
        .set(sender2_address.into(), key2.1, key2.2, value2)
        .unwrap();

    let deposit_sender2_tree: PoseidonSparseMerkleTree<NodeDataMemory> =
        deposit_sender2_tree.into();
    let sender2_deposit_root = deposit_sender2_tree.get(&sender2_address.into()).unwrap();
    dbg!(sender2_deposit_root);

    let merge_inclusion_proof2 = deposit_sender2_tree.find(&sender2_address.into()).unwrap();

    let deposit_tx_hash = HashOut::rand();
    dbg!(&deposit_tx_hash);
    let mut deposit_tree = PoseidonSparseMerkleTree::new(node_data, Default::default());
    deposit_tree
        .set(deposit_tx_hash.into(), sender2_deposit_root)
        .unwrap();
    let merge_inclusion_proof1 = deposit_tree.find(&deposit_tx_hash.into()).unwrap();

    let merge_process_proof = sender2_user_asset_tree
        .set(key1.0, sender2_deposit_root)
        .unwrap();

    let default_hash = HashOut {
        elements: [F::ZERO; 4],
    };
    let default_inclusion_proof = SparseMerkleInclusionProof {
        root: default_hash.into(),
        found: false,
        key: default_hash.into(),
        value: default_hash.into(),
        not_found_key: default_hash.into(),
        not_found_value: default_hash.into(),
        siblings: vec![],
        is_old0: true,
    };
    let prev_block_header = BlockHeader {
        block_number: 0,
        prev_block_header_digest: default_hash,
        transactions_digest: default_hash,
        // deposit_digest: *sender2_deposit_root,
        deposit_digest: *deposit_tree.get_root(),
        proposed_world_state_digest: default_hash,
        approved_world_state_digest: default_hash,
        latest_account_digest: default_hash,
    };

    let merge_proof = MergeProof {
        is_deposit: true,
        diff_tree_inclusion_proof: (
            prev_block_header.clone(),
            merge_inclusion_proof1,
            merge_inclusion_proof2,
        ),
        merge_process_proof,
        latest_account_tree_inclusion_proof: default_inclusion_proof,
    };

    world_state_tree
        .set(sender2_address.into(), sender2_user_asset_tree.get_root())
        .unwrap();

    let mut sender2_user_asset_tree: LayeredLayeredPoseidonSparseMerkleTree<NodeDataMemory> =
        sender2_user_asset_tree.into();
    let proof1 = sender2_user_asset_tree
        .set(key1.0, key2.1, key2.2, zero)
        .unwrap();
    let proof2 = sender2_user_asset_tree
        .set(key1.0, key1.1, key1.2, zero)
        .unwrap();

    let proof3 = sender2_tx_diff_tree
        .set(key3.0, key3.1, key3.2, value3)
        .unwrap();
    let proof4 = sender2_tx_diff_tree
        .set(key4.0, key4.1, key4.2, value4)
        .unwrap();

    let sender2_input_witness = vec![proof1, proof2];
    let sender2_output_witness = vec![proof3, proof4];
    // dbg!(
    //     serde_json::to_string(&sender2_input_witness).unwrap(),
    //     serde_json::to_string(&sender2_output_witness).unwrap()
    // );

    let mut pw = PartialWitness::new();
    merge_and_purge_circuit
        .targets
        .merge_proof_target
        .set_witness(
            &mut pw,
            &[],
            *sender1_input_witness.first().unwrap().0.old_root,
        );
    merge_and_purge_circuit
        .targets
        .purge_proof_target
        .set_witness(
            &mut pw,
            sender1_account.address,
            &sender1_input_witness,
            &sender1_output_witness,
            *sender1_input_witness.first().unwrap().0.old_root,
        );

    println!("start proving: sender1_tx_proof");
    let start = Instant::now();
    let sender1_tx_proof = merge_and_purge_circuit.prove(pw).unwrap();
    let end = start.elapsed();
    println!("prove: {}.{:03} sec", end.as_secs(), end.subsec_millis());

    // dbg!(&sender1_tx_proof.public_inputs);

    match merge_and_purge_circuit.verify(sender1_tx_proof.clone()) {
        Ok(()) => println!("Ok!"),
        Err(x) => println!("{}", x),
    }

    let mut pw = PartialWitness::new();
    merge_and_purge_circuit
        .targets
        .merge_proof_target
        .set_witness(&mut pw, &[merge_proof.clone()], default_hash);
    merge_and_purge_circuit
        .targets
        .purge_proof_target
        .set_witness(
            &mut pw,
            sender2_account.address,
            &sender2_input_witness,
            &sender2_output_witness,
            sender2_input_witness.first().unwrap().0.old_root.0.clone(),
        );

    println!("start proving: sender2_tx_proof");
    let start = Instant::now();
    let sender2_tx_proof = merge_and_purge_circuit.prove(pw).unwrap();
    let end = start.elapsed();
    println!("prove: {}.{:03} sec", end.as_secs(), end.subsec_millis());

    // dbg!(&sender2_tx_proof.public_inputs);

    match merge_and_purge_circuit.verify(sender2_tx_proof.clone()) {
        Ok(()) => println!("Ok!"),
        Err(x) => println!("{}", x),
    }

    let mut world_state_process_proofs = vec![];
    let mut user_tx_proofs = vec![];

    let sender1_world_state_process_proof = world_state_tree
        .set(sender1_address.into(), sender1_user_asset_tree.get_root())
        .unwrap();

    // dbg!(serde_json::to_string(&sender1_world_state_process_proof).unwrap());

    let sender2_world_state_process_proof = world_state_tree
        .set(sender2_address.into(), sender2_user_asset_tree.get_root())
        .unwrap();

    world_state_process_proofs.push(sender1_world_state_process_proof);
    user_tx_proofs.push(sender1_tx_proof.clone());
    world_state_process_proofs.push(sender2_world_state_process_proof);
    user_tx_proofs.push(sender2_tx_proof.clone());

    let zkdsa_circuit = make_simple_signature_circuit();

    let mut pw = PartialWitness::new();
    zkdsa_circuit.targets.set_witness(
        &mut pw,
        sender1_account.private_key,
        *world_state_tree.get_root(),
    );

    println!("start proving: sender1_received_signature");
    let start = Instant::now();
    let sender1_received_signature = zkdsa_circuit.prove(pw).unwrap();
    let end = start.elapsed();
    println!("prove: {}.{:03} sec", end.as_secs(), end.subsec_millis());

    // dbg!(&sender1_received_signature.public_inputs);

    let mut pw = PartialWitness::new();
    zkdsa_circuit.targets.set_witness(
        &mut pw,
        sender2_account.private_key,
        *world_state_tree.get_root(),
    );

    println!("start proving: sender2_received_signature");
    let start = Instant::now();
    let sender2_received_signature = zkdsa_circuit.prove(pw).unwrap();
    let end = start.elapsed();
    println!("prove: {}.{:03} sec", end.as_secs(), end.subsec_millis());

    // dbg!(&sender2_received_signature.public_inputs);

    let user_transaction_input = UserTransactionInput {
        sender_address: sender2_account.address,
        merge_witnesses: merge_proof,
        purge_input_witnesses: sender2_input_witness.clone(),
        purge_output_witnesses: sender2_output_witness,
        old_user_asset_root: sender2_input_witness.first().unwrap().0.old_root.0,
    };
    let simple_signature_input = SimpleSignatureInput {
        private_key: Wrapper(sender2_account.private_key),
        message: world_state_tree.get_root(),
    };

    let user_transaction_input_str = serde_json::to_string(&user_transaction_input).unwrap();
    fs::write(
        "./data/user_transaction_input.json",
        &user_transaction_input_str,
    )
    .unwrap();
    let simple_signature_input_str = serde_json::to_string(&simple_signature_input).unwrap();
    fs::write(
        "./data/simple_signature_input.json",
        &simple_signature_input_str,
    )
    .unwrap();
}
