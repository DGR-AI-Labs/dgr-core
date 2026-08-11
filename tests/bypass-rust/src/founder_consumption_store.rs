//! {FOUNDER-AUTHORS}: T0 `ConsumptionStore` interface unit.
//!
//! Agents must not implement, complete, or replace this fail-closed stub.

use crate::before_tool_call::GuardFault;

pub trait ConsumptionStore {
    fn consume(&mut self, _authorization_reference: &[u8]) -> Result<(), GuardFault> {
        // {FOUNDER-AUTHORS}: signature and non-functional default only.
        unimplemented!("FounderImplementationRequired")
    }
}
