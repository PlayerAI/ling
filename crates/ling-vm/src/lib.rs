//! Verifier-gated reference virtual machine for `ling.bytecode/1.0`.
//!
//! The executor accepts only [`ling_bytecode::VerifiedProgramV1`]. It has no
//! parser, HIR, checker, filesystem, environment, or ambient host-capability
//! access.

mod execute;
mod fault;
mod host;
mod value;

pub use execute::{ExecutionLimits, execute_v1};
pub use fault::{
    ExecutionError, InternalExecutionError, RuntimeFault, RuntimeFaultKind, RuntimeResource,
};
pub use host::{ConsoleCapability, HostCapabilities, HostError, HostErrorCategory};
