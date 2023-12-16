use crate::database::Value;

/// Similar to `TryFrom`. Avoids problems with coherence and has an additional `E` type.
pub trait Decode<C, E>: Sized
where
  E: From<crate::Error>,
{
  /// See [Value].
  type Value<'value>: Value;

  /// Performs the conversion.
  fn decode(input: Self::Value<'_>) -> Result<Self, E>;
}

impl Decode<(), crate::Error> for &str {
  type Value<'value> = ();

  #[inline]
  fn decode(_: Self::Value<'_>) -> Result<Self, crate::Error> {
    Ok("")
  }
}

impl Decode<(), crate::Error> for u32 {
  type Value<'value> = ();

  #[inline]
  fn decode(_: Self::Value<'_>) -> Result<Self, crate::Error> {
    Ok(0)
  }
}

impl Decode<(), crate::Error> for u64 {
  type Value<'value> = ();

  #[inline]
  fn decode(_: Self::Value<'_>) -> Result<Self, crate::Error> {
    Ok(0)
  }
}

impl<C, E, T> Decode<C, E> for Option<T>
where
  E: From<crate::Error>,
  T: Decode<C, E>,
{
  type Value<'value> = T::Value<'value>;

  #[inline]
  fn decode(input: Self::Value<'_>) -> Result<Self, E> {
    Ok(if input.is_null() { None } else { Some(T::decode(input)?) })
  }
}
