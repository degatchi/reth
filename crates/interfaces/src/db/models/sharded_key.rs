//! Sharded key

use crate::db::{
    table::{Decode, Encode},
    Error,
};
use reth_primitives::TxNumber;

/// Sometimes data can be too big to be saved for a single key. This helps out by dividing the data
/// into different shards. Example:
///
/// `Address | 200` -> data is from transaction 0 to 200.
///
/// `Address | 300` -> data is from transaction 201 to 300.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ShardedKey<T> {
    /// The key for this type.
    pub key: T,
    /// Highest tx number to which `value` is related to.
    pub highest_tx_number: TxNumber,
}

impl<T> ShardedKey<T> {
    /// Creates a new `ShardedKey<T>`.
    pub fn new(key: T, highest_tx_number: TxNumber) -> Self {
        ShardedKey { key, highest_tx_number }
    }
}

impl<T> Encode for ShardedKey<T>
where
    T: Encode,
    Vec<u8>: From<<T as Encode>::Encoded>,
{
    type Encoded = Vec<u8>;

    fn encode(self) -> Self::Encoded {
        let mut buf: Vec<u8> = Encode::encode(self.key).into();
        buf.extend_from_slice(&self.highest_tx_number.to_be_bytes());
        buf
    }
}

impl<T> Decode for ShardedKey<T>
where
    T: Decode,
{
    fn decode<B: Into<bytes::Bytes>>(value: B) -> Result<Self, Error> {
        let value: bytes::Bytes = value.into();
        let tx_num_index = value.len() - 8;

        let highest_tx_number = u64::from_be_bytes(
            value.as_ref()[tx_num_index..].try_into().map_err(|_| Error::DecodeError)?,
        );
        let key = T::decode(value.slice(..tx_num_index))?;

        Ok(ShardedKey::new(key, highest_tx_number))
    }
}
