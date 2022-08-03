#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod functions;
mod types;
use types::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{
    pallet_prelude::*,
    traits::{Contains, Currency},
};
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::traits::ReservableCurrency;

    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Used to pay the datas stored.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The storage containing the artists/candidates.
        type Artists: Contains<Self::AccountId>;

        /// The cost of storing one byte of data.
        #[pallet::constant]
        type CostPerByte: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type MaxDefaultStringLength: Get<u32>;

        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    // The pallet's runtime storage items.
    // https://docs.substrate.io/v3/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn get_artist_infos)]
    pub type ArtistMetadatas<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Metadatas<
            BoundedVec<u8, T::MaxDefaultStringLength>,
            BoundedVec<u8, T::MaxDescriptionLength>,
            BoundedVec<u8, T::MaxDefaultStringLength>,
            BoundedVec<u8, T::MaxDefaultStringLength>,
        >,
        ValueQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        UpdatedAlias(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDefaultStringLength>>,
        ),
        UpdatedBio(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDescriptionLength>>,
        ),
        UpdatedProfilePic(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDefaultStringLength>>,
        ),
        UpdatedTwitter(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDefaultStringLength>>,
        ),
        UpdatedFacebook(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDefaultStringLength>>,
        ),
        UpdatedInstagram(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDefaultStringLength>>,
        ),
        UpdatedSpotify(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDefaultStringLength>>,
        ),
        UpdatedAppleMusic(
            T::AccountId,
            Option<BoundedVec<u8, T::MaxDefaultStringLength>>,
        ),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The caller is not an artist or a candidate (not enough privilieges).
        NotArtistOrCandidate,
        /// This given string doesn't have a valid length.
        InvalidStringLength,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn update_alias(origin: OriginFor<T>, alias: Option<Vec<u8>>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(alias)?;

            // Bound the data and get the cost of the new datas
            let bounded_alias: Option<BoundedVec<u8, T::MaxDefaultStringLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.alias {
                Some(ref data) => Self::compute_cost(&data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.alias = bounded_alias.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedAlias(caller, bounded_alias));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_bio(origin: OriginFor<T>, bio: Option<Vec<u8>>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(bio)?;

            // Bound the data and get the cost of the new datas
            let bounded_bio: Option<BoundedVec<u8, T::MaxDescriptionLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.bio {
                Some(ref data) => Self::compute_cost(&data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.bio = bounded_bio.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedBio(caller, bounded_bio));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_profile_picture(
            origin: OriginFor<T>,
            picture_url: Option<Vec<u8>>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(picture_url)?;

            // Bound the data and get the cost of the new datas
            let bounded_pp: Option<BoundedVec<u8, T::MaxDefaultStringLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.profile_pic {
                Some(ref data) => Self::compute_cost(&data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.profile_pic = bounded_pp.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedProfilePic(caller, bounded_pp));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_twitter(origin: OriginFor<T>, username: Option<Vec<u8>>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(username)?;

            // Bound the data and get the cost of the new datas
            let bounded_username: Option<BoundedVec<u8, T::MaxDefaultStringLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.twitter {
                Some(ref data) => Self::compute_cost(&data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.profile_pic = bounded_username.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedTwitter(caller, bounded_username));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_facebook(
            origin: OriginFor<T>,
            profile_url: Option<Vec<u8>>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(profile_url)?;

            // Bound the data and get the cost of the new datas
            let bounded_url: Option<BoundedVec<u8, T::MaxDefaultStringLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.fb {
                Some(ref data) => Self::compute_cost(&data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.profile_pic = bounded_url.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedFacebook(caller, bounded_url));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_instagram(origin: OriginFor<T>, username: Option<Vec<u8>>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(username)?;

            // Bound the data and get the cost of the new datas
            let bounded_username: Option<BoundedVec<u8, T::MaxDefaultStringLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.instagram {
                Some(ref data) => Self::compute_cost(&data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.profile_pic = bounded_username.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedInstagram(caller, bounded_username));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_spotify(origin: OriginFor<T>, artist_id: Option<Vec<u8>>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(artist_id)?;

            // Bound the data and get the cost of the new datas
            let bounded_id: Option<BoundedVec<u8, T::MaxDefaultStringLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.spotify {
                Some(ref data) => Self::compute_cost(&data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.profile_pic = bounded_id.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedSpotify(caller, bounded_id));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_apple_music(
            origin: OriginFor<T>,
            username: Option<Vec<u8>>,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            // Ensure the caller is an artist or a candidate
            ensure!(Self::is_artist(&caller), Error::<T>::NotArtistOrCandidate);

            let result = Self::to_bounded_with_cost(username)?;

            // Bound the data and get the cost of the new datas
            let bounded_username: Option<BoundedVec<u8, T::MaxDefaultStringLength>> = result.0;
            let new_cost = result.1;

            let mut metadatas = <ArtistMetadatas<T>>::get(&caller);

            let old_cost = match metadatas.apple_music {
                Some(ref data) => Self::compute_cost(data),
                None => <BalanceOf<T>>::from(0u32),
            };

            if new_cost > old_cost {
                T::Currency::reserve(&caller, new_cost - old_cost)?;
            }
            if old_cost > new_cost {
                T::Currency::unreserve(&caller, old_cost - new_cost);
            }

            metadatas.profile_pic = bounded_username.clone();
            <ArtistMetadatas<T>>::insert(caller.clone(), metadatas);

            Self::deposit_event(Event::<T>::UpdatedAppleMusic(caller, bounded_username));

            Ok(())
        }
    }
}
