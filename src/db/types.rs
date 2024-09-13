use std::fmt::Display;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, Output, ToSql},
    sql_types::Text,
    sqlite::Sqlite,
};
use thiserror::Error;

// region:	  --- Timestamp
#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type=Text)]
pub struct Timestamp(time::OffsetDateTime);

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TimestampError {
    #[error("{0}")]
    ParseError(#[from] time::error::Parse),
}

impl Timestamp {
    pub fn new(timestamp: time::OffsetDateTime) -> Self {
        Timestamp(timestamp)
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use time::format_description::well_known::Rfc3339;

        // Can unwrap because using a well known format
        let v = self.0.format(&Rfc3339).unwrap();
        write!(f, "{}", v)
    }
}

impl TryFrom<&str> for Timestamp {
    type Error = TimestampError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use time::format_description::well_known::Rfc3339;

        let timestamp = time::OffsetDateTime::parse(value, &Rfc3339)?;
        Ok(Timestamp::new(timestamp))
    }
}

impl ToSql<Text, Sqlite> for Timestamp {
    fn to_sql(&self, out: &mut Output<Sqlite>) -> serialize::Result {
        let v = self.to_string();
        out.set_value(v);
        Ok(serialize::IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Timestamp
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let binding = String::from_sql(bytes)?;
        let timestamp = binding.as_str();
        Ok(Timestamp::try_from(timestamp)?)
    }
}
// endregion: --- Timestamp

// region:	  --- Date
#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type=Text)]
pub struct Date(time::Date);

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DateError {
    #[error("{0}")]
    ParseError(#[from] time::error::Parse),
}

impl Date {
    pub fn new(date: time::Date) -> Self {
        Self(date)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use time::format_description::well_known::Rfc3339;

        // Can unwrap because using a well known format
        let v = self.0.format(&Rfc3339).unwrap();
        write!(f, "{}", v)
    }
}

impl TryFrom<&str> for Date {
    type Error = DateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use time::format_description::well_known::Rfc3339;

        let date = time::Date::parse(value, &Rfc3339)?;
        Ok(Date::new(date))
    }
}

impl ToSql<Text, Sqlite> for Date {
    fn to_sql(&self, out: &mut Output<Sqlite>) -> serialize::Result {
        let v = self.to_string();
        out.set_value(v);
        Ok(serialize::IsNull::No)
    }
}

impl<DB> FromSql<Text, DB> for Date
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let binding = String::from_sql(bytes)?;
        let date = binding.as_str();
        Ok(Date::try_from(date)?)
    }
}
// endregion: --- Date

// region:	  --- Public ID
#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type=Text)]
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

impl TryFrom<&str> for PublicId {
    type Error = PublicIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 32 || !value.chars().all(|c| c.is_ascii_alphanumeric()) {
            Err(PublicIdError(value.to_string()))
        } else {
            Ok(PublicId(value.to_string()))
        }
    }
}

impl ToSql<Text, Sqlite> for PublicId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        <String as serialize::ToSql<Text, Sqlite>>::to_sql(&self.0, out)
    }
}

impl<DB> FromSql<Text, DB> for PublicId
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let string = String::from_sql(bytes)?;
        Ok(PublicId::try_from(string.as_str())?)
    }
}
// endregion: --- Public ID

// region: --- Constrained String
#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
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

impl<const MAX_LEN: usize> TryFrom<&str> for ConstrainedString<MAX_LEN> {
    type Error = ConstrainedStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ConstrainedString::new(value)
    }
}

impl<const MAX_LEN: usize, DB> ToSql<Text, DB> for ConstrainedString<MAX_LEN>
where
    DB: Backend,
    String: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        <String as ToSql<Text, DB>>::to_sql(&self.0, out)
    }
}

impl<const MAX_LEN: usize, DB> FromSql<Text, DB> for ConstrainedString<MAX_LEN>
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let string = String::from_sql(bytes)?;
        Ok(ConstrainedString::try_from(string.as_str())?)
    }
}
// endregion: --- Constrained String

pub type PlanName = ConstrainedString<128>;

pub type UserName = ConstrainedString<128>;

pub type Description = ConstrainedString<1024>;
