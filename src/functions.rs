use super::*;

impl<T: Config> Pallet<T> {
    /// Return true if an account is an artist or a candidate artist.
    pub fn is_artist(account: &T::AccountId) -> bool {
        T::Artists::contains(account)
    }

    /// Get the total cost to store a given data by multiplying the total of bytes with the cost per byte.
    pub fn compute_cost(data: &Vec<u8>) -> BalanceOf<T> {
        let bytes_count = <BalanceOf<T>>::from(data.len() as u32);
        bytes_count * T::CostPerByte::get()
    }

    pub fn to_bounded_with_cost<S: Get<u32>>(
        opt: Option<Vec<u8>>,
    ) -> Result<(Option<BoundedVec<u8, S>>, BalanceOf<T>), Error<T>> {
        match opt {
            Some(a) => {
                let new_cost = Self::compute_cost(&a);

                return Ok((
                    Some(a.try_into().map_err(|_| Error::<T>::InvalidStringLength)?),
                    new_cost,
                ));
            }
            None => {
                let new_cost = <BalanceOf<T>>::from(0u32);
                return Ok((None, new_cost));
            }
        };
    }
}
