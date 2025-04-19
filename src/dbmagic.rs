use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use surrealdb::{engine::local::Db, RecordId, Surreal};

static DB_INSTANCE: OnceCell<Surreal<Db>> = OnceCell::new();

pub fn init_db(db: Surreal<Db>) {
    DB_INSTANCE.set(db);
}

pub struct DbHandle;

impl Deref for DbHandle {
    type Target = Surreal<Db>;

    fn deref(&self) -> &Self::Target {
        DB_INSTANCE
            .get()
            .expect("Impossible: Database not initialized")
    }
}

pub static DB: DbHandle = DbHandle;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: RecordId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Count {
    pub count: i32,
}
