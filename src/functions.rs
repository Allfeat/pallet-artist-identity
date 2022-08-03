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
}
