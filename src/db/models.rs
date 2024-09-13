use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::plans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Plan {
    pub id: i32,
    pub public_id: super::types::PublicId,
    pub name: super::types::PlanName,
    pub description: Option<super::types::Description>,
    pub ctime: super::types::Timestamp,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub plan_id: i32,
    pub public_id: super::types::PublicId,
    pub name: super::types::UserName,
    pub ctime: super::types::Timestamp,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::dates)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Date {
    pub id: i32,
    pub user_id: i32,
    pub date: super::types::Date,
    pub ctime: super::types::Timestamp,
}
