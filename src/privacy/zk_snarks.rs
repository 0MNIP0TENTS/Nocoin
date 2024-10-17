pub fn generate_zk_proof(transaction: &Transaction, disclose: bool) -> ZKProof {
    let proof = zk_smart_contract_library::generate_proof(transaction);

    if disclose {
        zk_smart_contract_library::add_disclosure(&proof, transaction.get_auditable_data());
    }

    proof
}

pub fn verify_zk_proof(proof: &ZKProof, disclosed: bool) -> bool {
    if disclosed {
        zk_smart_contract_library::verify_proof_with_disclosure(proof)
    } else {
        zk_smart_contract_library::verify_proof(proof)
    }
}

pub fn batch_transactions_for_rollup(transactions: Vec<Transaction>) -> ZKRollup {
    let proofs: Vec<ZKProof> = transactions.into_iter().map(|tx| generate_zk_proof(&tx, false)).collect();
    zk_rollup::batch_proofs(proofs)
}

pub fn verify_zk_rollup(rollup: &ZKRollup) -> bool {
    zk_rollup::verify_rollup(rollup)
}
