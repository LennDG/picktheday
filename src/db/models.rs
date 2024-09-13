use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{self, Output, ToSql},
    sql_types::{self, Text},
    sqlite::Sqlite,
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::plans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Plan {
    pub id: i32,
    pub public_id: String,
    pub name: String,
    pub description: Option<String>,
    pub ctime: Timestamp,
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type=Text)]
pub struct Timestamp(time::OffsetDateTime);

impl ToSql<Text, Sqlite> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let v = self.0.to_string();
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
        use time::format_description::well_known::Rfc3339;

        let binding = String::from_sql(bytes)?;
        let timestamp = binding.as_str();
        Ok(Timestamp(time::OffsetDateTime::parse(timestamp, &Rfc3339)?))
    }
}
