use diesel::table;

table! {
    owner_events (id) {
        id -> Int4,
        event -> Varchar,
    }
}
