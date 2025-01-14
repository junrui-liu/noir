use acvm::acir::native_types::Expression;
use noirc_errors::Location;

use crate::{
    errors::{RuntimeError, RuntimeErrorKind},
    ssa::{
        acir_gen::{acir_mem::AcirMem, internal_var_cache::InternalVarCache, InternalVar},
        context::SsaContext,
        mem::{self, ArrayId, MemArray},
        node::NodeId,
    },
    Evaluator,
};

// Returns a variable corresponding to the element at the provided index in the array
// Returns an error if index is constant and out-of-bound.
pub(crate) fn evaluate(
    array_id: ArrayId,
    index: NodeId,
    acir_mem: &mut AcirMem,
    var_cache: &mut InternalVarCache,
    location: Option<Location>,
    evaluator: &mut Evaluator,
    ctx: &SsaContext,
) -> Result<InternalVar, RuntimeError> {
    let mem_array = &ctx.mem[array_id];
    let index = var_cache.get_or_compute_internal_var_unwrap(index, evaluator, ctx);
    evaluate_with(mem_array, &index, acir_mem, location, evaluator)
}

// Same as evaluate(), but using MemArray and InternalVar instead of ArrayId and NodeId
pub(crate) fn evaluate_with(
    array: &MemArray,
    index: &InternalVar,
    acir_mem: &mut AcirMem,
    location: Option<Location>,
    evaluator: &mut Evaluator,
) -> Result<InternalVar, RuntimeError> {
    if let Some(idx) = index.to_const() {
        let idx = mem::Memory::as_u32(idx);
        // Check to see if the index has gone out of bounds
        let array_length = array.len;
        if idx >= array_length {
            return Err(RuntimeError {
                location,
                kind: RuntimeErrorKind::ArrayOutOfBounds {
                    index: idx as u128,
                    bound: array_length as u128,
                },
            });
        }

        let array_element = acir_mem.load_array_element_constant_index(array, idx);
        if let Some(element) = array_element {
            return Ok(element);
        }
    }

    let w = evaluator.add_witness_to_cs();
    acir_mem.add_to_trace(&array.id, index.to_expression(), w.into(), Expression::zero());
    Ok(InternalVar::from_witness(w))
}
