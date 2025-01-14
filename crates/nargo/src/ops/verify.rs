use crate::{end_timer, start_timer};
use acvm::acir::{circuit::Circuit, native_types::WitnessMap};
use acvm::ProofSystemCompiler;

pub fn verify_proof<B: ProofSystemCompiler>(
    backend: &B,
    common_reference_string: &[u8],
    circuit: &Circuit,
    proof: &[u8],
    public_inputs: WitnessMap,
    verification_key: &[u8],
) -> Result<bool, B::Error> {
    let verify_time = start_timer!(|| "Verifying");
    // TODO(#1569): update from not just accepting `false` once we get nargo to interop with dynamic backend
    let result = backend.verify_with_vk(
        common_reference_string,
        proof,
        public_inputs,
        circuit,
        verification_key,
        false,
    );
    end_timer!(verify_time);
    result
}
