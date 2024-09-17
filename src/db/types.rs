use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{self, Output, ToSql},
    sql_types::{Integer, Text},
};
use thiserror::Error;

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

impl ToSql<Text, Pg> for PublicId {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <String as serialize::ToSql<Text, Pg>>::to_sql(&self.0, out)
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

// region:	  --- ID

#[derive(Debug, Clone, Copy, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type=Integer)]
pub struct Id(i32);

impl ToSql<Integer, Pg> for Id {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        <i32 as serialize::ToSql<Integer, Pg>>::to_sql(&self.0, out)
    }
}

impl<DB> FromSql<Integer, DB> for Id
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        let id = i32::from_sql(bytes)?;
        Ok(Id(id))
    }
}
// endregion: --- ID

pub type PlanId = Id;

pub type UserId = Id;

pub type DateId = Id;
