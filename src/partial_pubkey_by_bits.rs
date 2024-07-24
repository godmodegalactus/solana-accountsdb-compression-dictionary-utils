use serde::{de::Visitor, Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PartialPubkeyByBits {
    pub nb_bits: u8,
    pub bits: u8,
}

impl<'de> Deserialize<'de> for PartialPubkeyByBits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BytesVisitor {}

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = PartialPubkeyByBits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a bytes of size {SIZE:?}")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(PartialPubkeyByBits {
                    nb_bits: v[0],
                    bits: v[1],
                })
            }
        }
        Ok(deserializer.deserialize_bytes(BytesVisitor {})?)
    }
}

impl Serialize for PartialPubkeyByBits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&[self.nb_bits, self.bits])
    }
}

impl PartialPubkeyByBits {
    pub fn new(value: Pubkey, nb_bits: u8) -> Self {
        if nb_bits > 8 {
            panic!("nb_bits should be less than 8")
        }
        let mask = create_bit_mask(nb_bits);
        Self {
            nb_bits,
            bits: value.to_bytes()[0] | mask,
        }
    }
}

pub fn create_bit_mask(nb_bits: u8) -> u8 {
    if nb_bits == 0 {
        0
    } else if nb_bits == 8 {
        255
    } else {
        (1 << nb_bits) - 1
    }
}

#[test]
fn test_mask() {
    assert_eq!(create_bit_mask(0), 0);
    assert_eq!(create_bit_mask(1), 1);
    assert_eq!(create_bit_mask(2), 3);
    assert_eq!(create_bit_mask(3), 7);
    assert_eq!(create_bit_mask(4), 15);
    assert_eq!(create_bit_mask(5), 31);
    assert_eq!(create_bit_mask(6), 63);
    assert_eq!(create_bit_mask(7), 127);
    assert_eq!(create_bit_mask(8), 255);
}
