use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[mockall::automock]
pub trait TransactionProvider {
    fn transaction<T, E, F>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<T, E> + 'static,
        T: 'static,
        E: From<anyhow::Error> + From<diesel::result::Error> + 'static;
}
