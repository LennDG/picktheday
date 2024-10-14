use std::fmt::Display;

use crate::plan_page::UserWithDates;

pub struct Results(Vec<UserWithDates>);

impl Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
