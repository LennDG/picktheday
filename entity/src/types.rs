use derive_more::derive::Display;
use sea_orm::{DbErr, DeriveValueType, QueryResult, Value};
use thiserror::Error;

// region:	  --- Public ID
#[derive(Debug, Clone, PartialEq, Eq, DeriveValueType)]
pub struct PublicId(String);

#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid PublicId")]
pub struct PublicIdError(String);

impl PublicId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
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
// endregion: --- Public ID

// region:    --- Constrained String
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstrainedString<const MAX_LEN: usize>(String);

#[derive(Error, Debug, Clone, PartialEq)]
#[error("String too long: maximum {0} characters")]
pub struct ConstrainedStringError(usize);

impl<const MAX_LEN: usize> ConstrainedString<MAX_LEN> {
    pub fn new(text: &str) -> Result<Self, ConstrainedStringError> {
        if text.len() > MAX_LEN {
            Err(ConstrainedStringError(MAX_LEN))
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

pub type PlanName = ConstrainedString<128>;

pub type UserName = ConstrainedString<128>;

pub type Description = ConstrainedString<1024>;

// endregion: --- Constrained String

// region:	  --- ID

// THIS DOES NOT WORK BECAUSE SeaORM REALLY WANTS A i32 AS ID.

// #[derive(Debug, Clone, PartialEq, Eq, DeriveValueType)]
// pub struct Id(u64);

// impl TryFromU64 for Id {
//     fn try_from_u64(n: u64) -> Result<Self, DbErr> {
//         Ok(Id(n))
//     }
// }

// pub type PlanId = Id;

// pub type UserId = Id;

// pub type DateId = Id;

// endregion: --- ID
