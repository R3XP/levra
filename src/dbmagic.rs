use once_cell::sync::OnceCell;
use std::ops::Deref;
use surrealdb::{engine::local::Db, Surreal};

static DB_INSTANCE: OnceCell<Surreal<Db>> = OnceCell::new();

pub fn init_db(db: Surreal<Db>) {
    DB_INSTANCE.set(db);
}

pub struct DbHandle;

impl Deref for DbHandle {
    type Target = Surreal<Db>;

    fn deref(&self) -> &Self::Target {
        DB_INSTANCE.get().expect("Database not initialized")
    }
}

pub static DB: DbHandle = DbHandle;
