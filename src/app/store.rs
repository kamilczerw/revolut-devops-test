use surrealdb::{engine::local::Db, Surreal};

#[derive(Clone)]
pub(crate) struct Store {
    pub db: Surreal<Db>,
}

/// Store is a wrapper around the database.
/// It is used for convenience to group all database operations in one place.
impl Store {
    pub(crate) fn new(db: surrealdb::Surreal<surrealdb::engine::local::Db>) -> Self {
        Store { db }
    }

    #[cfg(test)]
    pub(crate) async fn new_in_mem() -> anyhow::Result<Self> {
        let db = Surreal::new::<surrealdb::engine::local::Mem>(()).await?;
        db.use_ns("revolut-test").use_db("revolut").await?;

        Ok(Store { db })
    }
}
