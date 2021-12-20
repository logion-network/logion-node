use crate::{self as pallet_loc, RequesterOf};
use sp_core::hash::H256;
use frame_support::{parameter_types, traits::EnsureOrigin};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system as system;
use system::ensure_signed;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		LogionLoc: pallet_loc::{Module, Call, Storage, Event<T>},
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

pub const LOC_OWNER1: u64 = 1;
pub const LOC_OWNER2: u64 = 2;
pub const LOC_REQUESTER_ID: u64 = 3;
pub const LOC_REQUESTER: RequesterOf<Test> = RequesterOf::<Test>::Account(LOC_REQUESTER_ID);

pub struct LoAuthorityListMock;
impl EnsureOrigin<Origin> for LoAuthorityListMock {
    type Success = ();

    fn try_origin(o: Origin) -> std::result::Result<Self::Success, Origin> {
		let result = ensure_signed(o.clone());
        match result {
			Ok(who) => {
				if who == LOC_OWNER1 || who == LOC_OWNER2 {
					Ok(())
				} else {
					Err(o)
				}
			},
			Err(_) => Err(o)
		}
    }
}

impl pallet_loc::Config for Test {
	type LocId = u32;
	type Event = Event;
	type Hash = H256;
	type CreateOrigin = LoAuthorityListMock;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
