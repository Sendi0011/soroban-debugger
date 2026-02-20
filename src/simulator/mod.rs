/// Network and ledger state simulator
///
/// This module provides comprehensive network state simulation for Soroban debugging.
/// It allows users to:
/// - Load network snapshots from JSON files
/// - Configure mock ledger state (accounts, contracts, balances)
/// - Pre-deploy contract instances with populated storage
/// - Save and restore ledger state for iterative debugging

pub mod state;
pub mod loader;
pub mod snapshot;

pub use state::{
    NetworkSnapshot, LedgerMetadata, AccountState, ContractState, SimulatorError,
};
pub use loader::{SnapshotLoader, LoadedSnapshot};
pub use snapshot::{SnapshotManager, SnapshotDiff, AccountDiff, ContractDiff};
