use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, PooledConnection},
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type PostgresDb = PgConnection;
