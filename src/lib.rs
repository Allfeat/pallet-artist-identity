#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

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
pub use pallet::*;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::Event::UpdatedStyles;
    use frame_support::traits::ReservableCurrency;
    use pallet_music_styles::BoundedStyle;

    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_music_styles::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Used to pay the data stored.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The storage containing the artists/candidates.
        type Artists: Contains<Self::AccountId>;

        /// The cost of storing one byte of data.
        #[pallet::constant]
        type CostPerByte: Get<BalanceOf<Self>>;

        /// The maximum of Styles a user can register for his profile
        #[pallet::constant]
        type MaxRegisteredStyles: Get<u32>;

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
    #[pallet::getter(fn get_artist_metadata)]
    pub type ArtistMetadata<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, MetadataOf<T>, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UpdatedMetadata(T::AccountId, Option<Vec<u8>>),
        UpdatedStyles(T::AccountId, Vec<Vec<u8>>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The caller is not an artist or a candidate (not enough privileges).
        NotArtistOrCandidate,
        /// This given string doesn't have a valid length.
        InvalidStringLength,
        /// The number of given styles is higher than the maximum styles authorized for a profile.
        TooMuchStylesSpecified,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsic", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn update_alias(origin: OriginFor<T>, alias: Option<Vec<u8>>) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, alias)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_bio(origin: OriginFor<T>, bio: Option<Vec<u8>>) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, bio)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_profile_picture(
            origin: OriginFor<T>,
            url: Option<Vec<u8>>,
        ) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, url)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_twitter(origin: OriginFor<T>, username: Option<Vec<u8>>) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, username)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_facebook(origin: OriginFor<T>, url: Option<Vec<u8>>) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, url)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_instagram(origin: OriginFor<T>, username: Option<Vec<u8>>) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, username)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_spotify(origin: OriginFor<T>, artist_id: Option<Vec<u8>>) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, artist_id)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_apple_music(
            origin: OriginFor<T>,
            username: Option<Vec<u8>>,
        ) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, username)?;
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn update_music_styles(origin: OriginFor<T>, styles: Vec<Vec<u8>>) -> DispatchResult {
            let caller = Self::ensure_signed_artist(origin)?;
            let mut metadata = <ArtistMetadata<T>>::get(caller.clone());
            let styles_db = <pallet_music_styles::Styles<T>>::get();

            let mut bounded_styles: BoundedVec<BoundedStyle<T>, T::MaxRegisteredStyles> =
                Default::default();

            let mut total_styles_cost: BalanceOf<T> = Default::default();

            for style in &styles {
                let s = pallet_music_styles::Pallet::<T>::to_bounded_style(style.clone())?;

                if !styles_db.contains_key(&s)
                    && styles_db
                        .values()
                        .find(|style| style.contains(&s))
                        .is_none()
                {
                    return Err(pallet_music_styles::Error::<T>::StyleNotFound)?;
                }

                total_styles_cost = total_styles_cost + Self::compute_cost(Some(style.clone()));

                bounded_styles
                    .try_push(s)
                    .map_err(|_| Error::<T>::TooMuchStylesSpecified)?;
            }

            let mut old_cost: BalanceOf<T> = Default::default();
            for style in metadata.music_styles {
                let unbounded_style: Vec<u8> = style.into();
                old_cost = old_cost + Self::compute_cost(Some(unbounded_style));
            }

            if total_styles_cost > old_cost {
                T::Currency::reserve(&caller, total_styles_cost - old_cost)?;
            }
            if old_cost > total_styles_cost {
                T::Currency::unreserve(&caller, old_cost - total_styles_cost);
            }

            metadata.music_styles = bounded_styles;
            <ArtistMetadata<T>>::insert(caller.clone(), metadata);

            Self::deposit_event(UpdatedStyles(caller, styles));

            Ok(())
        }
    }
}
