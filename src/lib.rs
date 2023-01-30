#![cfg_attr(not(feature = "std"), no_std)]

use allfeat_support::prelude::*;
use allfeat_support::traits::music::style::MutateMusicStyles;
use allfeat_support::types::music::style::MaxNameLength;
use frame_support::traits::ReservableCurrency;
use frame_support::{pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Saturating;
use sp_std::prelude::*;

pub use pallet::*;
use types::*;
use weights::WeightInfo;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod functions;
mod types;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Used to pay the data stored.
        type Currency: ReservableCurrency<Self::AccountId>;

        type StylesProvider: InspectMusicStyles<StyleName = MusicStyleName>;

        /// The Origin emitted by an Artist call.
        type ArtistOrigin: EnsureOrigin<
            <Self as frame_system::Config>::RuntimeOrigin,
            Success = Self::AccountId,
        >;

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

        #[cfg(feature = "runtime-benchmarks")]
        type StylesHelper: MutateMusicStyles<StyleName = MusicStyleName>;

        /// Weight information for extrinsics in this pallet.
        type Weights: WeightInfo;
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
        /// A field has been updated for the artist.
        UpdatedMetadata {
            artist: T::AccountId,
            field: FieldName,
            new_data: Vec<u8>,
        },
        /// Music Styles has been updated for the artist.
        UpdatedStyles {
            artist: T::AccountId,
            new_styles: Vec<Vec<u8>>,
        },
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
        /// The given style name is not an existant style.
        InexistantStyle,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsic", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::Weights::update_alias(T::MaxDefaultStringLength::get()))]
        pub fn update_alias(origin: OriginFor<T>, alias: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::Alias, alias)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::Weights::update_bio(T::MaxDescriptionLength::get()))]
        pub fn update_bio(origin: OriginFor<T>, bio: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::Bio, bio)?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::Weights::update_profile_picture(T::MaxDefaultStringLength::get()))]
        pub fn update_profile_picture(origin: OriginFor<T>, url: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::ProfilePic, url)?;
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::Weights::update_twitter(T::MaxDefaultStringLength::get()))]
        pub fn update_twitter(origin: OriginFor<T>, username: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::Twitter, username)?;
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::Weights::update_facebook(T::MaxDefaultStringLength::get()))]
        pub fn update_facebook(origin: OriginFor<T>, url: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::Facebook, url)?;
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(<T as Config>::Weights::update_instagram(T::MaxDefaultStringLength::get()))]
        pub fn update_instagram(origin: OriginFor<T>, username: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::Instagram, username)?;
            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(<T as Config>::Weights::update_spotify(T::MaxDefaultStringLength::get()))]
        pub fn update_spotify(origin: OriginFor<T>, artist_id: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::Spotify, artist_id)?;
            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(<T as Config>::Weights::update_apple_music(T::MaxDefaultStringLength::get()))]
        pub fn update_apple_music(origin: OriginFor<T>, username: Vec<u8>) -> DispatchResult {
            Self::update_field(origin, FieldName::AppleMusic, username)?;
            Ok(())
        }

        #[pallet::call_index(8)]
        #[pallet::weight(<T as Config>::Weights::update_music_styles(
            T::MaxDefaultStringLength::get(),
            <MaxNameLength as Get<u32>>::get()
        ))]
        pub fn update_music_styles(origin: OriginFor<T>, styles: Vec<Vec<u8>>) -> DispatchResult {
            let caller = Self::ensure_signed_artist(origin)?;
            let mut metadata = <ArtistMetadata<T>>::get(caller.clone());

            let mut bounded_styles: StyleListOf<T> = Default::default();

            let mut total_styles_cost: BalanceOf<T> = Default::default();

            for style in &styles {
                let s = T::StylesProvider::exist_from(style.clone())
                    .map_err(|_| Error::<T>::InvalidStringLength)?;

                total_styles_cost = total_styles_cost.saturating_add(Self::compute_cost(&style));

                match s {
                    Some(bounded) => {
                        bounded_styles
                            .try_push(bounded)
                            .map_err(|_| Error::<T>::TooMuchStylesSpecified)?;
                    }
                    None => return Err(Error::<T>::InexistantStyle)?,
                }
            }

            let mut old_cost: BalanceOf<T> = Default::default();
            for style in metadata.music_styles {
                let unbounded_style: Vec<u8> = style.into();
                old_cost = old_cost + Self::compute_cost(&unbounded_style);
            }

            if total_styles_cost > old_cost {
                T::Currency::reserve(&caller, total_styles_cost - old_cost)?;
            }
            if old_cost > total_styles_cost {
                T::Currency::unreserve(&caller, old_cost - total_styles_cost);
            }

            metadata.music_styles = bounded_styles;
            <ArtistMetadata<T>>::insert(caller.clone(), metadata);

            Self::deposit_event(Event::<T>::UpdatedStyles {
                artist: caller,
                new_styles: styles,
            });

            Ok(())
        }
    }
}
