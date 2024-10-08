use derive_more::derive::Display;
use sea_orm::{DbErr, DeriveValueType, QueryResult, Value};
use serde::{Deserialize, Deserializer};
use thiserror::Error;

// region:	  --- Public ID
#[derive(Debug, Clone, PartialEq, Eq, Display, DeriveValueType, Hash)]
pub struct PublicId(String);

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0:.120} is not a valid PublicId")]
pub struct PublicIdError(String);

impl PublicId {
    pub fn new(public_id: &str) -> Result<Self, PublicIdError> {
        if public_id.len() <= 32
            && !public_id.is_empty()
            && public_id.chars().all(char::is_alphanumeric)
        {
            Ok(PublicId(public_id.to_string()))
        } else {
            Err(PublicIdError(public_id.to_string()))
        }
    }
}

impl Default for PublicId {
    fn default() -> Self {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        let generated_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        Self(generated_string)
    }
}

impl<'de> Deserialize<'de> for PublicId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::new(&s).map_err(serde::de::Error::custom)
    }
}

pub fn deserialize_public_id_option<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<PublicId>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;

    match opt {
        Some(s) if s.is_empty() => Ok(None), // Treat empty string as None
        Some(s) => PublicId::new(&s)
            .map(Some)
            .map_err(serde::de::Error::custom), // Call PublicId::new directly
        None => Ok(None),
    }
}

// endregion: --- Public ID

// region:    --- Constrained String
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstrainedString<const MAX_LEN: usize>(String);

#[derive(Error, Debug, Clone)]
pub enum ConstrainedStringError {
    #[error("String too long: maximum {0} characters")]
    ConstrainedStringTooLong(usize),

    #[error("String empty")]
    ConstrainedStringEmpty(),
}

#[derive(Error, Debug, Clone)]
#[error("String too long: maximum {0} characters")]
pub struct ConstrainedStringTooLong(usize);

#[derive(Error, Debug, Clone)]
#[error("String empty")]
pub struct ConstrainedStringEmpty();

impl<const MAX_LEN: usize> ConstrainedString<MAX_LEN> {
    pub fn new(text: &str) -> Result<Self, ConstrainedStringError> {
        if text.len() > MAX_LEN {
            Err(ConstrainedStringError::ConstrainedStringTooLong(MAX_LEN))
        } else if text.is_empty() {
            Err(ConstrainedStringError::ConstrainedStringEmpty())
        } else {
            Ok(Self(text.to_string()))
        }
    }
}

impl<const MAX_LEN: usize> From<ConstrainedString<MAX_LEN>> for Value {
    fn from(value: ConstrainedString<MAX_LEN>) -> Self {
        value.0.into()
    }
}

impl<const MAX_LEN: usize> sea_orm::TryGetable for ConstrainedString<MAX_LEN> {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &QueryResult,
        idx: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        let string_value = <String as sea_orm::TryGetable>::try_get_by(res, idx)?;

        // Try to create the ConstrainedString with a length check
        ConstrainedString::new(&string_value)
            .map_err(|err| sea_orm::TryGetError::DbErr(DbErr::Type(err.to_string())))
    }
}

impl<const MAX_LEN: usize> sea_orm::sea_query::ValueType for ConstrainedString<MAX_LEN> {
    fn try_from(v: Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        let string_value = <String as sea_orm::sea_query::ValueType>::try_from(v)?;

        // Try to create the ConstrainedString with a length check
        ConstrainedString::new(&string_value).map_err(|_| sea_orm::sea_query::ValueTypeErr)
    }

    fn type_name() -> String {
        stringify!(StringVec).to_owned()
    }

    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::String
    }

    fn column_type() -> sea_orm::sea_query::ColumnType {
        <String as sea_orm::sea_query::ValueType>::column_type()
    }
}

impl<const MAX_LEN: usize> sea_orm::sea_query::Nullable for ConstrainedString<MAX_LEN> {
    fn null() -> sea_orm::Value {
        sea_orm::Value::String(None)
    }
}

impl<'de, const MAX_LEN: usize> Deserialize<'de> for ConstrainedString<MAX_LEN> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::new(&s).map_err(|err| serde::de::Error::custom(format!("{}", err)))
    }
}

pub type PlanName = ConstrainedString<128>;

pub type UserName = ConstrainedString<128>;

pub type Description = ConstrainedString<1024>;

// endregion: --- Constrained String
