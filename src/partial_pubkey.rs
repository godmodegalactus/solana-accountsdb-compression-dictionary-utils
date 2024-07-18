use std::collections::HashMap;

use serde::{de::Visitor, Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PartialPubkey<const SIZE: usize, const OFFSET: usize = 0>([u8; SIZE]);

impl<'de, const SIZE: usize, const OFFSET: usize> Deserialize<'de> for PartialPubkey<SIZE, OFFSET> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BytesVisitor<const SIZE: usize> {
        }

        impl<'de,  const SIZE: usize> Visitor<'de> for BytesVisitor<SIZE> {
            type Value = [u8; SIZE];

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a bytes of size {SIZE:?}")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                Ok(v[0..SIZE].try_into().unwrap())
            }
        }   
        let bytes = deserializer.deserialize_bytes(BytesVisitor{})?;
        Ok(Self(bytes))
    }
}

impl<const SIZE: usize, const OFFSET: usize> Serialize for PartialPubkey<SIZE, OFFSET> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<const SIZE: usize, const OFFSET: usize> From<Pubkey> for PartialPubkey<SIZE, OFFSET> {
    fn from(value: Pubkey) -> Self {
        Self(value.to_bytes()[OFFSET..OFFSET+SIZE].try_into().unwrap())
    }
}

impl<const SIZE: usize, const OFFSET: usize> From<&Pubkey> for PartialPubkey<SIZE, OFFSET> {
    fn from(value: &Pubkey) -> Self {
        Self(value.to_bytes()[OFFSET..OFFSET+SIZE].try_into().unwrap())
    }
}

impl<const SIZE: usize, const OFFSET: usize> PartialPubkey<SIZE, OFFSET> {
    pub fn to_bytes(&self) -> &[u8; SIZE] {
        &self.0
    }
}

pub type DictionaryMap = HashMap<PartialPubkey<4>, Vec<u8>>;