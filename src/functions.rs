use super::*;
use frame_support::traits::ReservableCurrency;

impl<T: Config> Pallet<T> {
    /// Ensure that the caller signed the transaction and is an Artist or a Candidate
    pub fn ensure_signed_artist(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError> {
        let caller = T::ArtistOrigin::ensure_origin(origin)?;
        Ok(caller)
    }

    /// Get the total cost to store a given data by multiplying the total of bytes with the cost per byte.
    pub fn compute_cost(data: &Vec<u8>) -> BalanceOf<T> {
        let bytes_count = <BalanceOf<T>>::from(data.len() as u32);
        bytes_count * T::CostPerByte::get()
    }

    /// Perform all mandatory actions to update a metadata field
    pub fn update_field(origin: OriginFor<T>, field: FieldName, value: Vec<u8>) -> DispatchResult {
        let caller = Self::ensure_signed_artist(origin)?;
        let metadata = <ArtistMetadata<T>>::get(caller.clone());
        let bounded_value = Self::into_field_for(value.clone(), &field)?;

        Self::pay_or_refund_cost_difference(&caller, &metadata, &field, value.clone())?;
        Self::update_metadata(&caller, metadata, bounded_value.clone())?;
        Self::deposit_event(Event::<T>::UpdatedMetadata {
            artist: caller.clone(),
            field,
            new_data: value,
        });

        Ok(())
    }

    /// Get raw value of the specified field of the specified account id
    pub fn get_raw_field_for(metadata: MetadataOf<T>, field: &FieldName) -> Vec<u8> {
        match field {
            FieldName::Alias => metadata.alias.into_inner(),
            FieldName::Bio => metadata.bio.into_inner(),
            FieldName::ProfilePic => metadata.profile_pic.into_inner(),
            FieldName::Twitter => metadata.twitter.into_inner(),
            FieldName::Facebook => metadata.facebook.into_inner(),
            FieldName::Instagram => metadata.instagram.into_inner(),
            FieldName::Spotify => metadata.spotify.into_inner(),
            FieldName::AppleMusic => metadata.apple_music.into_inner(),
        }
    }

    /// Convert a raw value of a field to a `FieldType`
    pub fn into_field_for(
        value: Vec<u8>,
        field: &FieldName,
    ) -> Result<FieldType<T>, DispatchError> {
        let on_err = |_| Error::<T>::InvalidStringLength;
        let result: FieldType<T> = match field {
            FieldName::Alias => Field::Alias(value.try_into().map_err(on_err)?),
            FieldName::Bio => Field::Bio(value.try_into().map_err(on_err)?),
            FieldName::ProfilePic => Field::ProfilePic(value.try_into().map_err(on_err)?),
            FieldName::Twitter => Field::Twitter(value.try_into().map_err(on_err)?),
            FieldName::Facebook => Field::Facebook(value.try_into().map_err(on_err)?),
            FieldName::Instagram => Field::Instagram(value.try_into().map_err(on_err)?),
            FieldName::Spotify => Field::Spotify(value.try_into().map_err(on_err)?),
            FieldName::AppleMusic => Field::AppleMusic(value.try_into().map_err(on_err)?),
        };
        Ok(result)
    }

    /// Update the given field of the given metadata object for an account id and update it on-chain
    fn update_metadata(
        caller: &T::AccountId,
        mut metadata: MetadataOf<T>,
        value: FieldType<T>,
    ) -> DispatchResult {
        match value {
            Field::Alias(v) => metadata.alias = v,
            Field::Bio(v) => metadata.bio = v,
            Field::ProfilePic(v) => metadata.profile_pic = v,
            Field::Twitter(v) => metadata.twitter = v,
            Field::Facebook(v) => metadata.facebook = v,
            Field::Instagram(v) => metadata.instagram = v,
            Field::Spotify(v) => metadata.spotify = v,
            Field::AppleMusic(v) => metadata.apple_music = v,
        };

        <ArtistMetadata<T>>::insert(caller, metadata);
        Ok(())
    }

    /// Compute if the cost for storing the new data is less or more than before for the specified field of a specified metadata object and pay or refund the caller
    fn pay_or_refund_cost_difference(
        caller: &T::AccountId,
        metadata: &MetadataOf<T>,
        field: &FieldName,
        value: Vec<u8>,
    ) -> DispatchResult {
        let new_cost = Self::compute_cost(&value);
        let old_cost = Self::compute_cost(&Self::get_raw_field_for(metadata.clone(), field));

        if new_cost > old_cost {
            T::Currency::reserve(&caller, new_cost - old_cost)?;
        }
        if old_cost > new_cost {
            T::Currency::unreserve(&caller, old_cost - new_cost);
        }

        Ok(())
    }
}
