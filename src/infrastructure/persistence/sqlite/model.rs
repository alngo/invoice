use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::owner_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct OwnerEvent {
    pub id: i32,
    pub event: String,
}
