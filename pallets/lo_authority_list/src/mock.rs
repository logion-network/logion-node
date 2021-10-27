use crate as pallet_lo_authority_list;
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
		LoAuthorityList: pallet_lo_authority_list::{Module, Call, Storage, Event<T>},
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

pub const MANAGER: u64 = 1;

pub struct EnsureManagerOriginMock;
impl EnsureOrigin<Origin> for EnsureManagerOriginMock {
    type Success = ();

    fn try_origin(o: Origin) -> std::result::Result<Self::Success, Origin> {
		let result = ensure_signed(o.clone());
        match result {
			Ok(who) => {
				if who == MANAGER {
					Ok(())
				} else {
					Err(o)
				}
			},
			Err(_) => Err(o)
		}
    }
}

impl pallet_lo_authority_list::Config for Test {
	type AddOrigin = EnsureManagerOriginMock;
	type RemoveOrigin = EnsureManagerOriginMock;
	type Event = Event;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
