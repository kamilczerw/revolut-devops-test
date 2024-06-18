use surrealdb::{engine::local::Db, Surreal};

#[derive(Clone)]
pub(crate) struct Store {
    pub db: Surreal<Db>,
}

impl Store {
    pub(crate) fn new(db: surrealdb::Surreal<surrealdb::engine::local::Db>) -> Self {
        Store { db }
    }
}
