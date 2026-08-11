//! {FOUNDER-AUTHORS}: T0 S2 durable-local consumption-store unit.
//!
//! Agents must not implement, complete, or replace this fail-closed stub.

use crate::before_tool_call::GuardFault;
use crate::founder_consumption_store::ConsumptionStore;

#[derive(Debug, Default)]
pub struct S2ConsumptionStore;

impl ConsumptionStore for S2ConsumptionStore {
    fn consume(&mut self, _authorization_reference: &[u8]) -> Result<(), GuardFault> {
        // {FOUNDER-AUTHORS}: signature and non-functional default only.
        unimplemented!("FounderImplementationRequired")
    }
}
