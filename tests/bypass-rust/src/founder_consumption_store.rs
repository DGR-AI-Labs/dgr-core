//! {FOUNDER-AUTHORS}: T0 `ConsumptionStore` interface unit.
//!
//! The default implementation fails closed until a concrete store supplies
//! durable single-use consumption.

use crate::before_tool_call::GuardFault;

#[derive(Debug, Eq, PartialEq)]
pub enum ConsumeOutcome {
    Consumed,
    AlreadyConsumed,
    Faulted(GuardFault),
}

pub trait ConsumptionStore {
    fn consume(&mut self, _authorization_reference: &[u8]) -> ConsumeOutcome {
        // {FOUNDER-AUTHORS}: a missing implementation must never permit execution.
        ConsumeOutcome::Faulted(GuardFault::FounderImplementationRequired)
    }
}
