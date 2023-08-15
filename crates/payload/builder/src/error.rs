//! Error types emitted by types or implementations of this crate.

use reth_primitives::H256;
use revm_primitives::EVMError;
use tokio::sync::oneshot;

/// Possible error variants during payload building.
#[derive(Debug, thiserror::Error)]
pub enum PayloadBuilderError {
    /// Thrown whe the parent block is missing.
    #[error("missing parent block {0:?}")]
    MissingParentBlock(H256),
    /// An oneshot channels has been closed.
    #[error("sender has been dropped")]
    ChannelClosed,
    /// Other internal error
    #[error(transparent)]
    Internal(#[from] reth_interfaces::Error),
    /// Unrecoverable error during evm execution.
    #[error("evm execution error: {0:?}")]
    EvmExecutionError(EVMError<reth_interfaces::Error>),
    /// Thrown if the payload requests withdrawals before Shanghai activation.
    #[error("withdrawals set before Shanghai activation")]
    WithdrawalsBeforeShanghai,
    /// Thrown when a transaction fails to convert to a
    /// [reth_primitives::TransactionSignedEcRecovered].
    #[cfg(feature = "optimism")]
    #[error("failed to convert deposit transaction to TransactionSignedEcRecovered")]
    TransactionEcRecoverFailed,
}

impl From<oneshot::error::RecvError> for PayloadBuilderError {
    fn from(_: oneshot::error::RecvError) -> Self {
        PayloadBuilderError::ChannelClosed
    }
}
