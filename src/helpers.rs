use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, SeqAccess};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

pub fn string_or_list<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that wraps string types in a Vec and deserializes Vecs.
    struct StringOrList<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrList<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_seq<S>(self, seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            // deserialize the sequence
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrList(PhantomData))
}

