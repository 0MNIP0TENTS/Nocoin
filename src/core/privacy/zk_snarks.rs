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

// Parallelized ZK proof generation for improved performance
use rayon::prelude::*;  // Parallel processing library

pub fn generate_zk_proofs_parallel(transactions: &[Transaction], disclose: bool) -> Vec<ZKProof> {
    transactions.par_iter()
        .map(|transaction| generate_zk_proof(transaction, disclose))
        .collect()
}
