use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::parameter_types;
use frame_support::traits::{ConstU64, SortedMembers};
use frame_system as system;
use frame_system::EnsureSignedBy;
use pallet_artists;
use pallet_balances;
use scale_info::TypeInfo;
use sp_core::{ConstU32, RuntimeDebug, H256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
use system::EnsureRoot;

use mock_artists::*;

/// This mocking is adapted to the environnement of the Allfeat chain.
use crate::{self as pallet_artist_identity};

#[derive(
    Encode,
    Decode,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    MaxEncodedLen,
    TypeInfo,
    RuntimeDebug,
)]
pub enum TestId {
    Foo,
    Bar,
    Baz,
}

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
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
    type RuntimeOrigin = RuntimeOrigin;
    type Nonce = u64;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type MaxHolds = ();
}

parameter_types! {
    // We use small max values for testing purpose
    pub const CreationDepositAmount: u64 = 10;
    pub const MaxArtists: u32 = 5;
    pub const MaxCandidates: u32 = 10;
    pub const NameMaxLength: u32 = 20;
}

impl pallet_artists::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Origin = RuntimeOrigin;
    type Call = RuntimeCall;
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
    type RuntimeEvent = RuntimeEvent;
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
    type RuntimeEvent = RuntimeEvent;
    type AdminOrigin = EnsureRoot<Self::AccountId>;
    type Weights = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(include_styles: bool) -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    // Give 100000 tokens to the 5 first accounts
    let config: pallet_balances::GenesisConfig<Test> = pallet_balances::GenesisConfig {
        balances: (0..5)
            .collect::<Vec<u64>>()
            .iter()
            .map(|&account_id| (account_id, 100000))
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
