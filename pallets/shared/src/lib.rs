#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{Parameter, dispatch::GetDispatchInfo, traits::{UnfilteredDispatchable, Vec}};

pub trait CreateRecoveryCallFactory<Origin, AccountId, BlockNumber> {
    type Call: Parameter + UnfilteredDispatchable<Origin = Origin> + GetDispatchInfo;

    fn build_create_recovery_call(legal_officers: Vec<AccountId>, threshold: u16, delay_period: BlockNumber) -> Self::Call;
}

pub trait LocQuery<AccountId> {
    fn has_closed_identity_locs(account: &AccountId, legal_officer: &Vec<AccountId>) -> bool;
}
