use std::time::Duration;
use serde::de::{Deserializer, Error, Unexpected, Visitor};
use serde::ser::{Serializer};
use std::fmt;

pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error> 
  where D: Deserializer<'de>
{
  struct V;

  impl<'de2> Visitor<'de2> for V {
    type Value = Duration;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
      fmt.write_str("a duration")
    }

    fn visit_str<E>(self, v: &str) -> Result<Duration, E>
        where E: Error
    {
      humantime::parse_duration(v).map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
    }
  }

  d.deserialize_str(V)
}

pub fn serialize<'ser, S>(t: &std::time::Duration, s: S) ->Result<S::Ok, S::Error>
  where S: Serializer 
{
  s.serialize_str(&format!("{}", humantime::format_duration(*t)))
}