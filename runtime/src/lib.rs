#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::KeyTypeId, H160, OpaqueMetadata, H256};
use sp_io::hashing::sha2_256;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount,
		NumberFor, One, Verify, OpaqueKeys,
	},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, MultiSignature, Percent
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::genesis_builder_helper::{build_config, create_default_config};
// A few exports that help ease life for downstream crates.
pub use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem,
		Randomness, StorageInfo, Contains, WrapperKeepOpaque, Imbalance,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
		IdentityFee, Weight,
	},
	StorageValue,
};
use frame_support::PalletId;
use frame_support::traits::{Currency, OnUnbalanced};
use frame_support::weights::ConstantMultiplier;
use frame_support::traits::tokens::{UnityAssetBalanceConversion, PayFromAccount};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

// Additional imports
use codec::{Decode, Encode, MaxEncodedLen};
use frame_system::EnsureRoot;
use logion_shared::{CreateRecoveryCallFactory, MultisigApproveAsMultiCallFactory, MultisigAsMultiCallFactory, DistributionKey, RewardDistributor as RewardDistributorTrait};
use pallet_logion_loc::{Hasher};
use pallet_multisig::Timepoint;
use scale_info::TypeInfo;
use sp_runtime::traits::IdentityLookup;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// LOC ID, compatible with UUIDs
pub type LocId = u128;

/// Ethereum Address
pub type EthereumAddress = H160;

/// Sponsorship ID, compatible with UUIDs
pub type SponsorshipId = u128;

/// A given token's total supply type
pub type TokenIssuance = u64;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}
}

// To learn more about runtime versioning and what each of the following value means:
//   https://substrate.dev/docs/en/knowledgebase/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("logion"),
	impl_name: create_runtime_str!("logion"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 164,
	impl_version: 2,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 5,
	state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

pub const NANO_LGNT: Balance = 1_000_000_000;
pub const MICRO_LGNT: Balance = 1_000 * NANO_LGNT;
pub const MILLI_LGNT: Balance = 1_000 * MICRO_LGNT;
pub const LGNT: Balance = 1_000 * MILLI_LGNT;

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub BlockWeights: frame_system::limits::BlockWeights =
			frame_system::limits::BlockWeights::with_sensible_defaults(
				Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
				NORMAL_DISPATCH_RATIO,
			);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		match call {
			RuntimeCall::Recovery(pallet_recovery::Call::create_recovery{ .. }) => false,
			RuntimeCall::Multisig(pallet_multisig::Call::approve_as_multi{ .. }) => false,
			RuntimeCall::Multisig(pallet_multisig::Call::as_multi{ .. }) => false,
			_ => true
		}
	}
}

mod weights;

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`SoloChainDefaultConfig`](`struct@frame_system::config_preludes::SolochainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = BaseCallFilter;
	/// The block type for the runtime.
	type Block = Block;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The type for storing how many extrinsics an account has signed.
	type Nonce = Nonce;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<32>;
	type AllowMultipleBlocksPerSlot = ConstBool<false>;

	#[cfg(feature = "experimental")]
	type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

impl pallet_grandpa::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = (); // Broken benchmark
	type MaxAuthorities = ConstU32<32>;
	type MaxNominators = ConstU32<0>;
	type MaxSetIdSessionEntries = ConstU64<0>;

	type KeyOwnerProof = sp_core::Void;
	type EquivocationReportSystem = ();
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

impl pallet_balances::Config for Runtime {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
	type FreezeIdentifier = [u8; 8];
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
}

parameter_types! {
    pub const InclusionFeesDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(35),
        community_treasury_percent: Percent::from_percent(30),
        logion_treasury_percent: Percent::from_percent(35),
        loc_owner_percent: Percent::from_percent(0),
    };

	// Inflation: I=0,05 (5%)
	// Total supply: N=10^9
	// Block rate: B=6 (Number of seconds between 2 blocks)
	// The reward can be calculated as follows: N * (I / (3600 * 24 * 365 / B))
	// We thus mint 10 LGNT every block
    pub const InflationAmount: Balance = 10 * LGNT;
    pub const InflationDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(35),
        community_treasury_percent: Percent::from_percent(30),
        logion_treasury_percent: Percent::from_percent(35),
        loc_owner_percent: Percent::from_percent(0),
    };

	pub const FileStorageByteFee: Balance = 2000 * NANO_LGNT; // 2.0 LGNT per MB -> 0.000002 LGNT per B
	pub const FileStorageEntryFee: Balance = 0;
	pub const FileStorageFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(80),
        community_treasury_percent: Percent::from_percent(20),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };

	pub const CertificateFee: Balance = 40 * MILLI_LGNT; // 0.04 LGNT per token
    pub const CertificateFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(20),
        community_treasury_percent: Percent::from_percent(80),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };

	pub const ValueFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(100),
        loc_owner_percent: Percent::from_percent(0),
    };

    pub const RecurentFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(95),
        loc_owner_percent: Percent::from_percent(5),
    };

    pub const IdentityLocLegalFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(100),
        loc_owner_percent: Percent::from_percent(0),
    };

    pub const OtherLocLegalFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(100),
    };
}

parameter_types! {
    pub const LogionTreasuryPalletId: PalletId = PalletId(*b"lg/lgtrs");
    pub LogionTreasuryAccountId: AccountId = LogionTreasuryPalletId::get().into_account_truncating();
    pub const CommunityTreasuryPalletId: PalletId = PalletId(*b"lg/cmtrs");
    pub CommunityTreasuryAccountId: AccountId = CommunityTreasuryPalletId::get().into_account_truncating();
}

type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

pub struct DealWithInclusionFees;

impl OnUnbalanced<NegativeImbalance> for DealWithInclusionFees {

	fn on_nonzero_unbalanced(fees: NegativeImbalance) {

		RewardDistributor::distribute(fees, InclusionFeesDistributionKey::get());
	}
}

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();

	// The multiplier is set such as inclusion fees are ~2 LGNT on average.
	// Spreadsheet in /docs/inclusion_fees.ods contains the model that lead
	// to this result.
	//
	// This value will probably have to be adjusted once we have more
	// usage statistics available.
	pub const WeightToFeeMultiplier: Balance = 5_089_484_898;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithInclusionFees>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = ConstantMultiplier<Balance, WeightToFeeMultiplier>;
	type LengthToFee = ConstantMultiplier<Balance, WeightToFeeMultiplier>;
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = weights::pallet_sudo::WeightInfo<Runtime>;
}

parameter_types! {
	#[derive(Debug, Eq, Clone, PartialEq, TypeInfo)]
	pub const MaxBaseUrlLen: u32 = 2000;
	pub const MaxWellKnownNodes: u32 = 100;
	#[derive(Debug, Eq, Clone, PartialEq, TypeInfo, PartialOrd, Ord)]
	pub const MaxPeerIdLength: u32 = 128;
}

parameter_types! {
	pub const MinAuthorities: u32 = 1;
}

impl pallet_validator_set::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddRemoveOrigin = EnsureRoot<AccountId>;
	type MinAuthorities = MinAuthorities;
	type WeightInfo = (); // Benchmark broken
}

parameter_types! {
	pub const Period: u32 = 2 * MINUTES;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = ValidatorSet;
	type RuntimeEvent = RuntimeEvent;
	type Keys = opaque::SessionKeys;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_validator_set::ValidatorOf<Self>;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>; // No benchmark available
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, Copy, MaxEncodedLen)]
pub enum Region {
    Europe,
}

impl sp_std::str::FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Europe" => Ok(Region::Europe),
            _ => Err(()),
        }
    }
}

impl Default for Region {

    fn default() -> Self {
        Self::Europe
    }
}

impl pallet_lo_authority_list::Config for Runtime {
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type Region = Region;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_lo_authority_list::WeightInfo<Runtime>;
	type MaxBaseUrlLen = MaxBaseUrlLen;
	type MaxNodes = MaxWellKnownNodes;
	type MaxPeerIdLength = MaxPeerIdLength;
}

parameter_types! {
	pub const MaxAccountLocs: u32 = 200;
	#[derive(TypeInfo)]
	pub const MaxLocMetadata: u32 = 50;
	#[derive(TypeInfo)]
	pub const MaxLocFiles: u32 = 50;
	#[derive(TypeInfo)]
	pub const MaxLocLinks: u32 = 50;
	pub const MaxCollectionItemFiles: u32 = 10;
	pub const MaxCollectionItemTCs: u32 = 10;
	pub const MaxTokensRecordFiles: u32 = 10;
}

pub struct SHA256;
impl Hasher<H256> for SHA256 {

	fn hash(data: &Vec<u8>) -> H256 {
		let bytes = sha2_256(data);
		H256(bytes)
	}
}

impl pallet_logion_loc::Config for Runtime {
	type LocId = LocId;
	type RuntimeEvent = RuntimeEvent;
	type Hash = Hash;
	type Hasher = SHA256;
	type IsLegalOfficer = LoAuthorityList;
	type CollectionItemId = Hash;
	type TokensRecordId = Hash;
	type MaxAccountLocs = MaxAccountLocs;
	type MaxLocMetadata = MaxLocMetadata;
	type MaxLocFiles = MaxLocFiles;
	type MaxLocLinks = MaxLocLinks;
	type MaxCollectionItemFiles = MaxCollectionItemFiles;
	type MaxCollectionItemTCs = MaxCollectionItemTCs;
	type MaxTokensRecordFiles = MaxTokensRecordFiles;
	type WeightInfo = weights::pallet_logion_loc::WeightInfo<Runtime>;
	type Currency = Balances;
	type FileStorageByteFee = FileStorageByteFee;
	type FileStorageEntryFee = FileStorageEntryFee;
	type RewardDistributor = RewardDistributor;
	type FileStorageFeeDistributionKey = FileStorageFeeDistributionKey;
	type EthereumAddress = EthereumAddress;
	type SponsorshipId = SponsorshipId;
	type CertificateFee = CertificateFee;
    type CertificateFeeDistributionKey = CertificateFeeDistributionKey;
    type TokenIssuance = TokenIssuance;
	type ValueFeeDistributionKey = ValueFeeDistributionKey;
	type CollectionItemFeeDistributionKey = RecurentFeeDistributionKey;
	type TokensRecordFeeDistributionKey = RecurentFeeDistributionKey;
	type IdentityLocLegalFeeDistributionKey = IdentityLocLegalFeeDistributionKey;
	type TransactionLocLegalFeeDistributionKey = OtherLocLegalFeeDistributionKey;
	type CollectionLocLegalFeeDistributionKey = OtherLocLegalFeeDistributionKey;
	#[cfg(feature = "runtime-benchmarks")]
	type LocIdFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type CollectionItemIdFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type TokensRecordIdFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type EthereumAddressFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type SponsorshipIdFactory = ();
}

parameter_types! {
	pub const RecoveryConfigDepositBase: u64 = 10;
	pub const RecoveryFrieldDepositFactor: u64 = 1;
	pub const MaxFriends: u16 = 3;
	pub const RecoveryDeposit: u64 = 10;
}

impl pallet_recovery::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ConfigDepositBase = RecoveryConfigDepositBase;
	type FriendDepositFactor = RecoveryFrieldDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
	type WeightInfo = weights::pallet_recovery::WeightInfo<Runtime>;
}

pub struct PalletRecoveryCreateRecoveryCallFactory;
impl CreateRecoveryCallFactory<RuntimeOrigin, AccountId, BlockNumber> for PalletRecoveryCreateRecoveryCallFactory {
	type Call = RuntimeCall;

	fn build_create_recovery_call(legal_officers: Vec<AccountId>, threshold: u16, delay_period: BlockNumber) -> RuntimeCall {
		RuntimeCall::Recovery(pallet_recovery::Call::create_recovery{ friends: legal_officers, threshold, delay_period })
	}
}

#[cfg(feature = "runtime-benchmarks")]
use pallet_verified_recovery::benchmarking::{
	SetupBenchmark,
};
#[cfg(feature = "runtime-benchmarks")]
pub struct VerifiedRecoverySetupBenchmark;
#[cfg(feature = "runtime-benchmarks")]
impl SetupBenchmark<AccountId> for VerifiedRecoverySetupBenchmark {

	fn setup() -> (AccountId, Vec<AccountId>) {
		let requester: AccountId = [0u8;32].into();
		Balances::make_free_balance_be(&requester, Balance::max_value());

		let loc_id1: LocId = 0;
		let legal_officer_id1 = LoAuthorityList::legal_officers()[0].clone();
		Self::setup_loc(loc_id1, &requester, &legal_officer_id1);

		let loc_id2: LocId = 1;
		let legal_officer_id2 = LoAuthorityList::legal_officers()[1].clone();
		Self::setup_loc(loc_id2, &requester, &legal_officer_id2);
		(
			requester,
			Vec::from([
				legal_officer_id1,
				legal_officer_id2,
			])
		)
	}
}
#[cfg(feature = "runtime-benchmarks")]
impl VerifiedRecoverySetupBenchmark {
	fn setup_loc(loc_id: LocId, requester: &AccountId, legal_officer_id: &AccountId) {
		let _ = LogionLoc::create_polkadot_identity_loc(
			RuntimeOrigin::signed(requester.clone()),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			ItemsParams::empty(),
		);
		let _ = LogionLoc::close(
			RuntimeOrigin::signed(legal_officer_id.clone()),
			loc_id,
			None,
			false,
		);
	}
}

impl pallet_verified_recovery::Config for Runtime {
	type LocId = LocId;
	type CreateRecoveryCallFactory = PalletRecoveryCreateRecoveryCallFactory;
	type LocQuery = LogionLoc;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_verified_recovery::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type SetupBenchmark = VerifiedRecoverySetupBenchmark;
}

parameter_types! {
	pub const MultiSigDepositBase: Balance = 500;
	pub const MultiSigDepositFactor: Balance = 100;
	pub const MaxSignatories: u16 = 20;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = MultiSigDepositBase;
	type DepositFactor = MultiSigDepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

pub struct PalletMultisigApproveAsMultiCallFactory;
impl MultisigApproveAsMultiCallFactory<RuntimeOrigin, AccountId, Timepoint<BlockNumber>> for PalletMultisigApproveAsMultiCallFactory {
	type Call = RuntimeCall;

	fn build_approve_as_multi_call(
		threshold: u16,
		other_signatories: Vec<AccountId>,
		maybe_timepoint: Option<Timepoint<BlockNumber>>,
		call_hash: [u8; 32],
		max_weight: Weight
	) -> RuntimeCall {
		RuntimeCall::Multisig(pallet_multisig::Call::approve_as_multi{ threshold, other_signatories, maybe_timepoint, call_hash, max_weight })
	}
}

pub struct PalletMultisigAsMultiCallFactory;
impl MultisigAsMultiCallFactory<RuntimeOrigin, AccountId, Timepoint<BlockNumber>> for PalletMultisigAsMultiCallFactory {
	type Call = RuntimeCall;

	fn build_as_multi_call(
		threshold: u16,
		other_signatories: Vec<AccountId>,
		maybe_timepoint: Option<Timepoint<BlockNumber>>,
		call: Box<Self::Call>,
		max_weight: Weight,
	) -> RuntimeCall {
		RuntimeCall::Multisig(pallet_multisig::Call::as_multi{ threshold, other_signatories, maybe_timepoint, call, max_weight })
	}
}

impl pallet_logion_vault::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type MultisigApproveAsMultiCallFactory = PalletMultisigApproveAsMultiCallFactory;
	type MultisigAsMultiCallFactory = PalletMultisigAsMultiCallFactory;
	type IsLegalOfficer = LoAuthorityList;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

#[cfg(feature = "runtime-benchmarks")]
use pallet_logion_vote::benchmarking::{
	LocSetup,
};
#[cfg(feature = "runtime-benchmarks")]
use logion_shared::IsLegalOfficer;
#[cfg(feature = "runtime-benchmarks")]
use pallet_logion_loc::ItemsParams;
#[cfg(feature = "runtime-benchmarks")]
pub struct VoteLocSetup;
#[cfg(feature = "runtime-benchmarks")]
impl LocSetup<LocId, AccountId> for VoteLocSetup {

	fn setup_vote_loc() -> (LocId, AccountId) {
		let loc_id: LocId = 0;
		let requester: AccountId = [0u8;32].into();
		Balances::make_free_balance_be(&requester, Balance::max_value());
		let legal_officer_id = LoAuthorityList::legal_officers()[0].clone();
		let _ = LogionLoc::create_polkadot_identity_loc(
			RuntimeOrigin::signed(requester),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			ItemsParams::empty(),
		);
		let _ = LogionLoc::close(
			RuntimeOrigin::signed(legal_officer_id.clone()),
			loc_id,
			None,
			false,
		);
		(loc_id, legal_officer_id)
	}
}


parameter_types! {
	#[derive(Debug, PartialEq, TypeInfo)]
	pub const MaxBallots: u32 = 12;
}

impl pallet_logion_vote::Config for Runtime {
	type LocId = LocId;
	type RuntimeEvent = RuntimeEvent;
	type IsLegalOfficer = LoAuthorityList;
	type LocValidity = LogionLoc;
	type LocQuery = LogionLoc;
	type LegalOfficerCreation = LoAuthorityList;
	type MaxBallots = MaxBallots;
	type WeightInfo = weights::pallet_logion_vote::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type LocSetup = VoteLocSetup;
}

parameter_types! {
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = 100 * LGNT;
    pub const SpendPeriod: BlockNumber = 1 * DAYS;
	pub const SpendPayoutPeriod: BlockNumber = 30 * DAYS;
}

type LogionTreasuryType = pallet_treasury::Instance1;
impl pallet_treasury::Config<LogionTreasuryType> for Runtime {
	type Currency = Balances;
	type ApproveOrigin = EnsureRoot<AccountId>;
	type RejectOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = LogionTreasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type SpendPeriod = SpendPeriod;
	type Burn = ();
	type PalletId = LogionTreasuryPalletId;
	type BurnDestination = ();
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>; // Benchmark broken
	type SpendFunds = ();
	type MaxApprovals = ConstU32<100>;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type AssetKind = ();
	type Beneficiary = AccountId;
	type BeneficiaryLookup = IdentityLookup<AccountId>;
	type Paymaster = PayFromAccount<Balances, LogionTreasuryAccountId>;
	type BalanceConverter = UnityAssetBalanceConversion;
	type PayoutPeriod = SpendPayoutPeriod;
}

type CommunityTreasuryType = pallet_treasury::Instance2;
impl pallet_treasury::Config<CommunityTreasuryType> for Runtime {
	type Currency = Balances;
	type ApproveOrigin = EnsureRoot<AccountId>;
	type RejectOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = CommunityTreasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type SpendPeriod = SpendPeriod;
	type Burn = ();
	type PalletId = CommunityTreasuryPalletId;
	type BurnDestination = ();
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>; // Benchmark broken
	type SpendFunds = ();
	type MaxApprovals = ConstU32<100>;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type AssetKind = ();
	type Beneficiary = AccountId;
	type BeneficiaryLookup = IdentityLookup<AccountId>;
	type Paymaster = PayFromAccount<Balances, CommunityTreasuryAccountId>;
	type BalanceConverter = UnityAssetBalanceConversion;
	type PayoutPeriod = SpendPayoutPeriod;
}

pub struct RewardDistributor;
impl logion_shared::RewardDistributor<NegativeImbalance, Balance, AccountId, RuntimeOrigin, LoAuthorityList>
    for RewardDistributor
{
    fn payout_community_treasury(reward: NegativeImbalance) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(&CommunityTreasuryPalletId::get().into_account_truncating(), reward);
		}
    }

	fn payout_logion_treasury(reward: NegativeImbalance) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(&LogionTreasuryPalletId::get().into_account_truncating(), reward);
		}
	}

	fn payout_to(reward: NegativeImbalance, account: &AccountId) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(account, reward);
		}
	}
}

impl pallet_block_reward::Config for Runtime {
    type Currency = Balances;
    type RewardAmount = InflationAmount;
    type RewardDistributor = RewardDistributor;
    type DistributionKey = InflationDistributionKey;
	type IsLegalOfficer = LoAuthorityList;
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime {
		System: frame_system = 0,
		// 1 was randomness collective flip which is considered as insecure: https://github.com/paritytech/substrate/pull/13301
		Timestamp: pallet_timestamp = 2,
		Balances: pallet_balances = 3,
		ValidatorSet: pallet_validator_set = 4,
		Session: pallet_session = 5,
		Aura: pallet_aura = 6,
		Grandpa: pallet_grandpa = 7,
		TransactionPayment: pallet_transaction_payment = 8,
		Sudo: pallet_sudo = 9,
		// 10 was NodeAuthorization
		Multisig:  pallet_multisig = 11,
		Recovery: pallet_recovery = 12,
		// 13 was Assets
		LoAuthorityList: pallet_lo_authority_list = 14,
		LogionLoc: pallet_logion_loc = 15,
		VerifiedRecovery: pallet_verified_recovery = 16,
		Vault: pallet_logion_vault = 17,
		Vote: pallet_logion_vote = 18,
		// 19 was Treasury
		BlockReward: pallet_block_reward = 20,
		LogionTreasury: pallet_treasury::<Instance1> = 21,
		CommunityTreasury: pallet_treasury::<Instance2> = 22,
		Utility: pallet_utility = 23,
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

parameter_types! {
	pub const LogionLocStr: &'static str = "LogionLoc";
}

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_grandpa, Grandpa]
		[pallet_lo_authority_list, LoAuthorityList]
		[pallet_logion_loc, LogionLoc]
		[pallet_logion_vote, Vote]
		[pallet_multisig, Multisig]
		[pallet_recovery, Recovery]
		[pallet_sudo, Sudo]
		[pallet_timestamp, Timestamp]
		[pallet_validator_set, ValidatorSet]
		[pallet_verified_recovery, VerifiedRecovery]
		[pallet_utility, Utility]
	);
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}

		fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
			Runtime::metadata_at_version(version)
		}

		fn metadata_versions() -> sp_std::vec::Vec<u32> {
			Runtime::metadata_versions()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> sp_consensus_grandpa::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: sp_consensus_grandpa::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: sp_consensus_grandpa::SetId,
			_authority_id: GrandpaId,
		) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_logion_loc::runtime_api::FeesApi<Block, Balance, TokenIssuance> for Runtime {
		fn query_file_storage_fee(num_of_entries: u32, tot_size: u32) -> Balance {
			LogionLoc::calculate_fee(num_of_entries, tot_size)
		}

		fn query_certificate_fee(token_issuance: TokenIssuance) -> Balance {
			LogionLoc::calculate_certificate_fee(token_issuance)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch};
			use sp_storage::TrackedStorageKey;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, BlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
		}
	}

	impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
		fn create_default_config() -> Vec<u8> {
			create_default_config::<RuntimeGenesisConfig>()
		}

		fn build_config(config: Vec<u8>) -> sp_genesis_builder::Result {
			build_config::<RuntimeGenesisConfig>(config)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::traits::WhitelistedStorageKeys;
	use sp_core::hexdisplay::HexDisplay;
	use std::collections::HashSet;

	#[test]
	fn check_whitelist() {
		let whitelist: HashSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
			.iter()
			.map(|e| HexDisplay::from(&e.key).to_string())
			.collect();

		// Block Number
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
		);
		// Total Issuance
		assert!(
			whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
		);
		// Execution Phase
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
		);
		// Event Count
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
		);
		// System Events
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
		);
	}
}
