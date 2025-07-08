use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub mod u8_hex_array {
    use super::*;

    pub fn serialize<S>(data: &[u8; 6], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(6))?;
        for byte in data.iter() {
            seq.serialize_element(byte)?; // 序列化为整数，不是字符串
        }
        seq.end()
    }

    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 6], D::Error>
    where
        D: Deserializer<'de>,
    {
        struct U8HexVisitor;

        impl<'de> Visitor<'de> for U8HexVisitor {
            type Value = [u8; 6];

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a list of 6 u8 values, decimal or 0x-prefixed hex")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<[u8; 6], A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = [0u8; 6];
                for i in 0..6 {
                    let value: serde_yaml::Value = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;

                    let parsed = match &value {
                        // YAML会将裸数字解析成Number，可以是十进制或十六进制
                        serde_yaml::Value::Number(n) => n
                            .as_u64()
                            .ok_or_else(|| de::Error::custom("invalid number"))?,
                        // 或者写成 "0x??" 的字符串，也接受
                        serde_yaml::Value::String(s) => {
                            if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X"))
                            {
                                u8::from_str_radix(hex, 16)
                                    .map_err(|_| de::Error::custom("invalid hex string"))?
                                    as u64
                            } else {
                                s.parse::<u64>()
                                    .map_err(|_| de::Error::custom("invalid decimal string"))?
                            }
                        }
                        _ => return Err(de::Error::custom("expected number or string")),
                    };

                    if value > (u8::MAX as u64).into() {
                        return Err(de::Error::custom("value out of range for u8"));
                    }

                    result[i] = parsed as u8;
                }
                Ok(result)
            }
        }

        deserializer.deserialize_seq(U8HexVisitor)
    }
}
