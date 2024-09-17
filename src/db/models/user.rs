use diesel::prelude::*;

use crate::db::types::{PlanId, PublicId, UserId, UserName};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: UserId,
    pub plan_id: PlanId,
    pub public_id: PublicId,
    pub name: UserName,
    pub ctime: time::OffsetDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub public_id: PublicId,
    pub plan_id: PlanId,
    pub name: UserName,
    pub ctime: time::OffsetDateTime,
}

impl NewUser {
    pub fn new(name: UserName, plan_id: PlanId) -> Self {
        NewUser {
            public_id: PublicId::new(),
            name,
            plan_id,
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
        models::plan::{NewPlan, Plan},
        schema::plans::public_id,
        types::{Description, PlanName, PublicId, UserName},
    };

    #[tokio::test]
    async fn test_create_user_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let mut conn = mm.get_conn();

        // -- Fixtures
        let fx_new_plan = NewPlan::new(
            PlanName::new("create_user_ok").unwrap(),
            Some(Description::new("Create user description").unwrap()),
        );

        let fx_name = UserName::new("create_user_ok").unwrap();

        // -- Setup
        use crate::db::schema::plans;
        let new_plan: Plan = diesel::insert_into(plans::table)
            .values(&fx_new_plan)
            .returning(Plan::as_returning())
            .get_result(&mut conn)
            .expect("Error creating plan");

        let fx_new_user = NewUser::new(fx_name.clone(), new_plan.id);

        use crate::db::schema::users;
        let new_user: User = diesel::insert_into(users::table)
            .values(&fx_new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .expect("Error creating user");

        // -- Check
        assert_eq!(fx_name, new_user.name);
        assert_eq!(new_plan.id, new_user.plan_id);

        // -- Cleanup
        let num_deleted = diesel::delete(plans::dsl::plans.filter(plans::id.eq(new_plan.id)))
            .execute(&mut conn)
            .expect("Error deleting plan");
        assert_eq!(num_deleted, 1);
        Ok(())
    }
}

// endregion:    --- Tests
