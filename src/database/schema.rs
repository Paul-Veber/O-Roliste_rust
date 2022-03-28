table! {
    tagsperso (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        pseudo -> Varchar,
        email -> Varchar,
        password -> Varchar,
        age -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    tagsperso,
    users,
);
