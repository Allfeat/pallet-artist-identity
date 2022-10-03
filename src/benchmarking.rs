//! Benchmarking setup for pallet-template

use frame_benchmarking::{benchmarks, whitelist_account};
#[allow(unused)]
use frame_support::dispatch::UnfilteredDispatchable;
use sp_runtime::traits::Bounded;

use super::*;

struct BenchmarkData<T: Config> {
    pub origin: T::Origin,
    pub account_id: T::AccountId,
    pub call_data: Vec<u8>,
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn setup_origin_and_data<T: Config>(data_length: usize) -> BenchmarkData<T> {
    let origin = T::ArtistOrigin::successful_origin();
    let account_id = T::ArtistOrigin::ensure_origin(origin.clone()).unwrap();
    whitelist_account!(account_id);
    T::Currency::make_free_balance_be(&account_id, BalanceOf::<T>::max_value());
    BenchmarkData {
        origin,
        account_id,
        call_data: vec![61; data_length],
    }
}

benchmarks! {
    where_clause { where T: Config }

    update_alias {
        let n in 0..T::MaxDefaultStringLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_alias { alias: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::Alias,
            new_data: data.call_data
        }.into());
    }

    update_bio {
        let n in 0..T::MaxDescriptionLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_bio { bio: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::Bio,
            new_data: data.call_data
        }.into());
    }

    update_profile_picture {
        let n in 0..T::MaxDefaultStringLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_profile_picture { url: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::ProfilePic,
            new_data: data.call_data
        }.into());
    }

    update_twitter {
        let n in 0..T::MaxDefaultStringLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_twitter { username: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::Twitter,
            new_data: data.call_data
        }.into());
    }

    update_facebook {
        let n in 0..T::MaxDefaultStringLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_facebook { url: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::Facebook,
            new_data: data.call_data
        }.into());
    }

    update_instagram {
        let n in 0..T::MaxDefaultStringLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_instagram { username: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::Instagram,
            new_data: data.call_data
        }.into());
    }

    update_spotify {
        let n in 0..T::MaxDefaultStringLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_spotify { artist_id: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::Spotify,
            new_data: data.call_data
        }.into());
    }

    update_apple_music {
        let n in 0..T::MaxDefaultStringLength::get();

        let data = setup_origin_and_data::<T>(n as usize);
        let call = Call::<T>::update_apple_music { username: data.call_data.clone() };
    }: { call.dispatch_bypass_filter(data.origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedMetadata {
            artist: data.account_id,
            field: FieldName::AppleMusic,
            new_data: data.call_data
        }.into());
    }

    update_music_styles {
        let n in 1..T::MaxRegisteredStyles::get();
        let x in 1..<T::StylesProvider as InspectMusicStyles>::StyleName::max_encoded_len()
            .saturating_sub(1) as u32;

        let mut styles: Vec<Vec<u8>> = Vec::new();
        for i in 0..n {
            styles.push(vec![0x61 + i as u8; x as usize])
        }
        for style in &styles {
            T::StylesHelper::add_parent_style(style.clone().try_into().unwrap())?;
        }

        let origin = T::ArtistOrigin::successful_origin();
        let account_id = T::ArtistOrigin::ensure_origin(origin.clone()).unwrap();
        whitelist_account!(account_id);
        T::Currency::make_free_balance_be(&account_id, BalanceOf::<T>::max_value());
        let call = Call::<T>::update_music_styles { styles: styles.clone() };
    }: { call.dispatch_bypass_filter(origin)? }
    verify {
        assert_last_event::<T>(Event::<T>::UpdatedStyles {
            artist: account_id,
            new_styles: styles
        }.into());
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(false), crate::mock::Test);
}
