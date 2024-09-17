use diesel::prelude::*;

use crate::db::types::{DateId, UserId};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::dates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Date {
    pub id: DateId,
    pub user_id: UserId,
    pub date: time::Date,
    pub ctime: time::OffsetDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::dates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewDate {
    pub user_id: UserId,
    pub date: time::Date,
    pub ctime: time::OffsetDateTime,
}

impl NewDate {
    pub fn new(date: time::Date, user_id: UserId) -> Self {
        NewDate {
            user_id,
            date,
            ctime: time::OffsetDateTime::now_utc(),
        }
    }
}

// region:    --- Tests
#[cfg(test)]
mod tests {

    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    use crate::db::{
        _dev_utils,
        models::{
            plan::{NewPlan, Plan},
            user::{NewUser, User},
        },
        schema::plans::public_id,
        types::{Description, PlanName, PublicId, UserName},
    };

    #[tokio::test]
    async fn test_create_date_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let mut conn = mm.get_conn();

        // -- Fixtures
        let fx_new_plan = NewPlan::new(
            PlanName::new("create_date_ok").unwrap(),
            Some(Description::new("Create date description").unwrap()),
        );

        let fx_user_name = UserName::new("create_date_ok").unwrap();

        // -- Setup
        use crate::db::schema::plans;
        let new_plan: Plan = diesel::insert_into(plans::table)
            .values(&fx_new_plan)
            .returning(Plan::as_returning())
            .get_result(&mut conn)
            .expect("Error creating plan");

        let fx_new_user = NewUser::new(fx_user_name.clone(), new_plan.id);

        use crate::db::schema::users;
        let new_user: User = diesel::insert_into(users::table)
            .values(&fx_new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .expect("Error creating user");

        let fx_date = NewDate::new(time::OffsetDateTime::now_utc().date(), new_user.id);

        use crate::db::schema::dates;
        let new_date: Date = diesel::insert_into(dates::table)
            .values(&fx_date)
            .returning(Date::as_returning())
            .get_result(&mut conn)
            .expect("Error creating user");

        // -- Check
        assert_eq!(fx_date.date, new_date.date);
        assert_eq!(new_user.id, new_date.user_id);

        // -- Cleanup
        let num_deleted = diesel::delete(plans::dsl::plans.filter(plans::id.eq(new_plan.id)))
            .execute(&mut conn)
            .expect("Error deleting plan");
        assert_eq!(num_deleted, 1);
        Ok(())
    }
}
// endregion: --- Tests
