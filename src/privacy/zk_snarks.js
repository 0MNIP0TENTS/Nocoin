// privacy.js

const snarkjs = require("snarkjs");

async function create_zk_proof(input) {
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, "circuit.wasm", "circuit_final.zkey");
    return { proof, publicSignals };
}

async function verify_zk_proof(proof, publicSignals) {
    const vkey = JSON.parse(fs.readFileSync("verification_key.json"));
    const isValid = await snarkjs.groth16.verify(vkey, publicSignals, proof);
    return isValid;
}
