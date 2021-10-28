use crate::{self as pallet_verified_recovery};
use logion_shared::{LocQuery, CreateRecoveryCallFactory};
use sp_core::hash::H256;
use frame_support::{parameter_types};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system as system;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		VerifiedRecovery: pallet_verified_recovery::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

pub struct CreateRecoveryCallFactoryMock;
impl CreateRecoveryCallFactory<<Test as system::Config>::Origin, <Test as system::Config>::AccountId, <Test as system::Config>::BlockNumber> for CreateRecoveryCallFactoryMock {
    type Call = Call;

    fn build_create_recovery_call(_legal_officers: Vec<<Test as system::Config>::AccountId>, _threshold: u16, _delay_period: <Test as system::Config>::BlockNumber) -> Self::Call {
        Call::System(frame_system::Call::remark(Vec::from([0u8])))
    }
}

pub const LEGAL_OFFICER_CLOSED_ID1: u64 = 1;
pub const LEGAL_OFFICER_CLOSED_ID2: u64 = 2;
pub const LEGAL_OFFICER_PENDING_OR_OPEN_ID1: u64 = 3;
pub const LEGAL_OFFICER_PENDING_OR_OPEN_ID2: u64 = 4;
pub const USER_ID: u64 = 5;

pub struct LocQueryMock;
impl LocQuery<<Test as system::Config>::AccountId> for LocQueryMock {
    fn has_closed_identity_locs(
		account: &<Test as system::Config>::AccountId,
		legal_officers: &Vec<<Test as system::Config>::AccountId>
	) -> bool {
        return *account == USER_ID && legal_officers[0] == LEGAL_OFFICER_CLOSED_ID1 && legal_officers[1] == LEGAL_OFFICER_CLOSED_ID2;
    }
}

impl pallet_verified_recovery::Config for Test {
	type CreateRecoveryCallFactory = CreateRecoveryCallFactoryMock;
	type LocQuery = LocQueryMock;
	type Event = Event;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
