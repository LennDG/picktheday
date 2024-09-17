// @generated automatically by Diesel CLI.

diesel::table! {
    dates (id) {
        id -> Int4,
        user_id -> Int4,
        date -> Date,
        ctime -> Timestamptz,
    }
}

diesel::table! {
    plans (id) {
        id -> Int4,
        #[max_length = 32]
        public_id -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 1024]
        description -> Nullable<Varchar>,
        ctime -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 32]
        public_id -> Varchar,
        plan_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        ctime -> Timestamptz,
    }
}

diesel::joinable!(dates -> users (user_id));
diesel::joinable!(users -> plans (plan_id));

diesel::allow_tables_to_appear_in_same_query!(dates, plans, users,);
