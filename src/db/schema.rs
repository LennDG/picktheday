// @generated automatically by Diesel CLI.

diesel::table! {
    dates (id) {
        id -> Integer,
        user_id -> Integer,
        date -> Text,
        ctime -> Text,
    }
}

diesel::table! {
    plans (id) {
        id -> Integer,
        public_id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        ctime -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        public_id -> Text,
        plan_id -> Integer,
        name -> Text,
        ctime -> Text,
    }
}

diesel::joinable!(dates -> users (user_id));
diesel::joinable!(users -> plans (plan_id));

diesel::allow_tables_to_appear_in_same_query!(
    dates,
    plans,
    users,
);
