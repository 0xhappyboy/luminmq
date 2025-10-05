pub mod common {
    use std::{collections::HashMap, sync::MutexGuard};

    pub fn get_keys_for_value<K, V>(map: MutexGuard<HashMap<K, V>>, value: V) -> Vec<K>
    where
        V: Eq,
        K: Clone,
    {
        let mut keys = Vec::<K>::new();
        for (k, v) in map.iter() {
            if v.eq(&value) {
                keys.push(k.clone());
            }
        }
        return keys;
    }
}

pub mod codec {
    use bincode::{Decode, Encode, config};
    // coding
    pub fn encode<T>(t: T) -> Vec<u8>
    where
        T: Encode,
    {
        let config = config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(&t, config).unwrap();
        bytes
    }
    // decoding
    pub fn decode<T>(bytes: &[u8]) -> Result<T, ()>
    where
        T: Decode<()>,
    {
        let config = config::standard();
        match bincode::decode_from_slice::<T, _>(&bytes[..], config) {
            Ok((decoded, len)) => return Ok(decoded),
            Err(e) => {
                return Err(());
            }
        }
    }
}
