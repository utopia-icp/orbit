use crate::{
    models::{Transfer, TransferId, WalletId},
    repositories::TransferRepository,
};
use candid::{CandidType, Deserialize};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a transfer list index in the system.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferWalletIndex {
    /// The wallet associated with the transfer.
    pub wallet_id: WalletId,
    /// The timestamp of the transfer creation.
    pub created_timestamp: Timestamp,
    /// The transfer associated with the wallet.
    pub transfer_id: TransferId,
}

#[derive(Clone, Debug)]
pub struct TransferWalletIndexCriteria {
    pub wallet_id: WalletId,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
    pub status: Option<String>,
}

impl TransferWalletIndex {
    /// The default criteria interval in nanoseconds (7 days).
    pub const DEFAULT_CRITERIA_INTERVAL_NS: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;

    pub fn to_transfer(&self) -> Transfer {
        TransferRepository::default()
            .get(&Transfer::key(self.transfer_id))
            .expect("Transfer not found")
    }
}

impl Transfer {
    pub fn to_index_by_wallet(&self) -> TransferWalletIndex {
        TransferWalletIndex {
            wallet_id: self.from_wallet,
            created_timestamp: self.created_timestamp,
            transfer_id: self.id,
        }
    }
}
