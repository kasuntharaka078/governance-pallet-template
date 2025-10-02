//! Mock runtime for testing the simple governance pallet.

use crate as pallet_simple_governance;
use frame_support::{
    derive_impl, parameter_types,
    traits::{OnFinalize, OnInitialize},
};
use sp_runtime::{
    traits::IdentityLookup, BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
#[frame_support::runtime]
mod test_runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask,
        RuntimeViewFunction
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type SimpleGovernance = pallet_simple_governance::Pallet<Test>;
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
}

parameter_types! {
    pub const MaxDescriptionLength: u32 = 256;
    pub const DefaultVotingPeriod: u64 = 100;
    pub const MaxProposalsPerBlock: u32 = 10;
}

impl pallet_simple_governance::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxDescriptionLength = MaxDescriptionLength;
    type DefaultVotingPeriod = DefaultVotingPeriod;
    type MaxProposalsPerBlock = MaxProposalsPerBlock;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    
    crate::GenesisConfig::<Test> {
        proposals: vec![
            // Add some initial proposals for testing if needed
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    
    t.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 1 {
            <System as OnFinalize<u64>>::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        <System as OnInitialize<u64>>::on_initialize(System::block_number());
        <SimpleGovernance as OnInitialize<u64>>::on_initialize(System::block_number());
    }
}