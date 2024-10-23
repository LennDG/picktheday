use std::fmt::Display;

use crate::plan_page::{calendar::ranked_dates, UserWithDates};

pub struct Results(Vec<UserWithDates>);

impl Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Check if any dates are picked by all:
        let ranked_dates = ranked_dates(&self.0);

        write!(f, "RESULTS!")
    }
}
