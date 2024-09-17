use diesel::prelude::*;

use crate::db::types::{Description, PlanId, PlanName, PublicId};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::plans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Plan {
    pub id: PlanId,
    pub public_id: PublicId,
    pub name: PlanName,
    pub description: Option<Description>,
    pub ctime: time::OffsetDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::plans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPlan {
    pub public_id: PublicId,
    pub name: PlanName,
    pub description: Option<Description>,
    pub ctime: time::OffsetDateTime,
}

impl NewPlan {
    pub fn new(name: PlanName, description: Option<Description>) -> Self {
        NewPlan {
            public_id: PublicId::new(),
            name,
            description,
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
        schema::plans::public_id,
        types::{Description, PlanName, PublicId, UserName},
    };

    #[tokio::test]
    async fn test_create_plan_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let mut conn = mm.get_conn();

        // -- Fixtures
        let fx_name = PlanName::new("create_plan_ok").unwrap();
        let fx_description = Description::new("Create plan description").unwrap();
        let fx_new_plan = NewPlan::new(fx_name.clone(), Some(fx_description));

        // -- Setup
        use crate::db::schema::plans::*;
        let new_plan: Plan = diesel::insert_into(table)
            .values(&fx_new_plan)
            .returning(Plan::as_returning())
            .get_result(&mut conn)
            .expect("Error creating plan");

        // -- Check
        assert_eq!(fx_name, new_plan.name);

        // -- Cleanup
        let num_deleted = diesel::delete(dsl::plans.filter(id.eq(new_plan.id)))
            .execute(&mut conn)
            .expect("Error deleting plan");
        assert_eq!(num_deleted, 1);
        Ok(())
    }
}
// endregion:    --- Tests
