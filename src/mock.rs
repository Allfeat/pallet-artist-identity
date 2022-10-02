use frame_support::traits::{ConstU16, ConstU64, SortedMembers};
use frame_system as system;
use frame_system::EnsureSignedBy;
use pallet_artists;
use pallet_balances;
use sp_core::H256;
use sp_runtime::{
    parameter_types,
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureRoot;

use mock_artists::*;

/// This mocking is adapted to the environnement of the Allfeat chain.
use crate::{
    self as pallet_artist_identity,
    mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        ArtistIdentity: pallet_artist_identity,
        Balances: pallet_balances,
        MusicStyles: pallet_music_styles,
        Artists: pallet_artists,
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
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
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    // We use small max values for testing purpose
    pub const CreationDepositAmount: u64 = 10;
    pub const MaxArtists: u32 = 5;
    pub const MaxCandidates: u32 = 10;
    pub const NameMaxLength: u32 = 20;
}

impl pallet_artists::Config for Test {
    type Event = Event;
    type Origin = Origin;
    type Call = Call;
    type Currency = Balances;
    type AdminOrigin = EnsureRoot<Self::AccountId>;
    type CreationDepositAmount = CreationDepositAmount;
    type NameMaxLength = NameMaxLength;
    type WeightInfo = ();
}

parameter_types! {
    pub const CostPerByte: u64 = 5;
    pub const MaxDefaultStringLength: u32 = 256;
    pub const MaxDescriptionLength: u32 = 521;
    pub const MaxRegisteredStyles: u32 = 3;
}

pub struct ArtistMock;
impl SortedMembers<u64> for ArtistMock {
    fn sorted_members() -> Vec<u64> {
        vec![0, 1, 2]
    }
}

impl pallet_artist_identity::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type StylesProvider = MusicStyles;
    type MaxDefaultStringLength = MaxDefaultStringLength;
    type ArtistOrigin = EnsureSignedBy<ArtistMock, Self::AccountId>;
    type CostPerByte = CostPerByte;
    type MaxDescriptionLength = MaxDescriptionLength;
    type MaxRegisteredStyles = MaxRegisteredStyles;
    #[cfg(feature = "runtime-benchmarks")]
    type StylesHelper = MusicStyles;
    type Weights = ();
}

parameter_types! {
    pub const MaxStyleCount: u32 = 3;
    pub const MaxSubStyleCount: u32 = 5;
    pub const StyleNameMaxLength: u32 = 64;
}

impl pallet_music_styles::Config for Test {
    type Event = Event;
    type AdminOrigin = EnsureRoot<Self::AccountId>;
    type Weights = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(include_styles: bool) -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    // Give 100 tokens to the 100 first accounts
    let config: pallet_balances::GenesisConfig<Test> = pallet_balances::GenesisConfig {
        balances: (0..100)
            .collect::<Vec<u64>>()
            .iter()
            .map(|&account_id| (account_id, 100))
            .collect(),
    };

    let artist_config: pallet_artists::GenesisConfig<Test> = pallet_artists::GenesisConfig {
        artists: vec![
            (ALICE.account_id, ALICE.name.into()),
            (BOB.account_id, BOB.name.into()),
        ],
        candidates: vec![(CHARLIE.account_id, CHARLIE.name.into())],
    };

    if include_styles {
        let styles_config: pallet_music_styles::GenesisConfig<Test> =
            pallet_music_styles::GenesisConfig {
                styles: vec![
                    ("Electro".into(), vec![]),
                    (
                        "Rap".into(),
                        vec!["Drill".into(), "Trap".into(), "Hardcore".into()],
                    ),
                    ("Rock".into(), vec!["Hardcore".into()]),
                ],
                phantom: Default::default(),
            };
        styles_config.assimilate_storage(&mut storage).unwrap();
    }

    config.assimilate_storage(&mut storage).unwrap();
    artist_config.assimilate_storage(&mut storage).unwrap();

    let mut ext: sp_io::TestExternalities = storage.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub mod mock_artists {
    use super::*;

    pub struct Artist {
        pub account_id: <Test as system::Config>::AccountId,
        pub name: &'static str,
        pub alias: &'static str,
        pub bio: &'static str,
        pub profile_pic: &'static str,
    }

    pub const ALICE: Artist = Artist {
        account_id: 0,
        name: "Alice",
        alias: "Alice_alias",
        bio: "A simple artist to test stuff !",
        profile_pic: "",
    };
    pub const BOB: Artist = Artist {
        account_id: 1,
        name: "Bob",
        alias: "Bob_alias",
        bio: "A simple artist to test stuff !",
        profile_pic: "",
    };
    pub const CHARLIE: Artist = Artist {
        account_id: 2,
        name: "Charlie",
        alias: "Charlie_alias",
        bio: "A simple artist to test stuff !",
        profile_pic: "",
    };
}
