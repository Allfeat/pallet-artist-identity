//! Autogenerated weights for `pallet_artist_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-22, STEPS: `50`, REPEAT: 30, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `DESKTOP-11NSJM8`, CPU: `Intel(R) Core(TM) i7-10700K CPU @ 3.80GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/allfeat
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_artist-identity
// --extrinsic
// *
// --steps
// 50
// --repeat
// 30
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get};
use frame_support::weights::constants::RocksDbWeight;
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn update_alias(_n: u32) -> Weight;
    fn update_bio(_n: u32) -> Weight;
    fn update_profile_picture(_n: u32) -> Weight;
    fn update_twitter(_n: u32) -> Weight;
    fn update_facebook(_n: u32) -> Weight;
    fn update_instagram(_n: u32) -> Weight;
    fn update_spotify(_n: u32) -> Weight;
    fn update_apple_music(_n: u32) -> Weight;
    fn update_music_styles(_n: u32, _x: u32) -> Weight;
}

/// Weight functions for `pallet_artist_identity`.
pub struct AllfeatWeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AllfeatWeightInfo<T> {
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_alias(n: u32, ) -> Weight {
        (21_379_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((13_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 512]`.
    fn update_bio(n: u32, ) -> Weight {
        (21_312_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_profile_picture(n: u32, ) -> Weight {
        (21_101_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((9_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_twitter(n: u32, ) -> Weight {
        (21_189_000 as Weight)
            // Standard Error: 0
            .saturating_add((7_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_facebook(n: u32, ) -> Weight {
        (20_929_000 as Weight)
            // Standard Error: 0
            .saturating_add((8_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_instagram(n: u32, ) -> Weight {
        (21_044_000 as Weight)
            // Standard Error: 0
            .saturating_add((6_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_spotify(n: u32, ) -> Weight {
        (21_059_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((9_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_apple_music(n: u32, ) -> Weight {
        (20_927_000 as Weight)
            // Standard Error: 0
            .saturating_add((7_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    // Storage: MusicStyles Styles (r:1 w:0)
    /// The range of component `n` is `[1, 5]`.
    /// The range of component `x` is `[1, 64]`.
    fn update_music_styles(n: u32, x: u32, ) -> Weight {
        (23_484_000 as Weight)
            // Standard Error: 66_000
            .saturating_add((711_000 as Weight).saturating_mul(n as Weight))
            // Standard Error: 2_000
            .saturating_add((22_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

use frame_support::weights::Weight;

impl WeightInfo for () {
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_alias(n: u32, ) -> Weight {
        (21_379_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((13_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 512]`.
    fn update_bio(n: u32, ) -> Weight {
        (21_312_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_profile_picture(n: u32, ) -> Weight {
        (21_101_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((9_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_twitter(n: u32, ) -> Weight {
        (21_189_000 as Weight)
            // Standard Error: 0
            .saturating_add((7_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_facebook(n: u32, ) -> Weight {
        (20_929_000 as Weight)
            // Standard Error: 0
            .saturating_add((8_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_instagram(n: u32, ) -> Weight {
        (21_044_000 as Weight)
            // Standard Error: 0
            .saturating_add((6_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_spotify(n: u32, ) -> Weight {
        (21_059_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((9_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    /// The range of component `n` is `[0, 128]`.
    fn update_apple_music(n: u32, ) -> Weight {
        (20_927_000 as Weight)
            // Standard Error: 0
            .saturating_add((7_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    // Storage: ArtistIdentity ArtistMetadata (r:1 w:1)
    // Storage: MusicStyles Styles (r:1 w:0)
    /// The range of component `n` is `[1, 5]`.
    /// The range of component `x` is `[1, 64]`.
    fn update_music_styles(n: u32, x: u32, ) -> Weight {
        (23_484_000 as Weight)
            // Standard Error: 66_000
            .saturating_add((711_000 as Weight).saturating_mul(n as Weight))
            // Standard Error: 2_000
            .saturating_add((22_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
