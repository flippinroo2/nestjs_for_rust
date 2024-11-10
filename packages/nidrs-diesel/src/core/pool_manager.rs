use std::{marker::Send, sync::Mutex};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    QueryResult,
};
use nidrs::AppResult;
use nidrs_extern::{
    anyhow,
    axum::{async_trait, http},
    tokio::task,
};

#[cfg(feature = "sqlite")]
pub mod sqlite {
    use crate::ConnectionDriver;

    use super::PoolManager;

    use diesel::SqliteConnection;

    use diesel::r2d2::ConnectionManager;

    use diesel::r2d2::Pool;
    use nidrs::injectable;

    use std::sync::Mutex;

    type TConnection = SqliteConnection;

    #[injectable()]
    pub struct SqlitePoolManager {
        pub pool: Option<Mutex<Pool<ConnectionManager<TConnection>>>>,
    }

    impl SqlitePoolManager {
        pub fn new<T: Into<String>>(url: T) -> SqlitePoolManager {
            let manager: ConnectionManager<TConnection> = ConnectionManager::<TConnection>::new(url);
            // Refer to the `r2d2` documentation for more methods to use
            // when building a connection pool
            let pool: Pool<ConnectionManager<TConnection>> =
                Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool"); // This is where the file:db.sqlite3 file is created if it doesn't exist... Was able to fix this by putting "file:src/db.sqlite3" instead.

            SqlitePoolManager { pool: Some(Mutex::new(pool)) }
        }
    }

    impl PoolManager for SqlitePoolManager {
        type Connection = TConnection;
        fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<TConnection>>>> {
            &self.pool
        }
    }

    impl From<SqlitePoolManager> for ConnectionDriver {
        fn from(val: SqlitePoolManager) -> Self {
            ConnectionDriver::Sqlite(val)
        }
    }
}

#[cfg(feature = "mysql")]
pub mod mysql {
    use crate::ConnectionDriver;

    use super::super::PoolManager;

    use diesel::MysqlConnection;

    use diesel::r2d2::ConnectionManager;

    use diesel::r2d2::Pool;
    use nidrs::injectable;

    use std::sync::Mutex;

    type TConnection = MysqlConnection;

    #[injectable()]
    pub struct MysqlPoolManager {
        pub pool: Option<Mutex<Pool<ConnectionManager<TConnection>>>>,
    }

    impl MysqlPoolManager {
        pub fn new<T: Into<String>>(url: T) -> MysqlPoolManager {
            let manager: ConnectionManager<TConnection> = ConnectionManager::<TConnection>::new(url);
            let pool: Pool<ConnectionManager<TConnection>> =
                Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

            MysqlPoolManager { pool: Some(Mutex::new(pool)) }
        }
    }

    impl PoolManager for MysqlPoolManager {
        type Connection = TConnection;
        fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<TConnection>>>> {
            &self.pool
        }
    }

    impl From<MysqlPoolManager> for ConnectionDriver {
        fn from(val: MysqlPoolManager) -> Self {
            ConnectionDriver::Mysql(val)
        }
    }
}

#[cfg(feature = "postgres")]
pub mod postgres {
    use crate::ConnectionDriver;

    use super::super::PoolManager;

    use diesel::PgConnection;

    use diesel::r2d2::ConnectionManager;

    use diesel::r2d2::Pool;
    use nidrs::injectable;

    use std::sync::Mutex;

    type TConnection = PgConnection;

    #[injectable()]
    #[derive(Default)]
    pub struct PostgresPoolManager {
        pub pool: Option<Mutex<Pool<ConnectionManager<TConnection>>>>,
    }

    impl PostgresPoolManager {
        pub fn new<T: Into<String>>(url: T) -> PostgresPoolManager {
            let manager: ConnectionManager<TConnection> = ConnectionManager::<TConnection>::new(url);
            let pool: Pool<ConnectionManager<TConnection>> =
                Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool");

            PostgresPoolManager { pool: Some(Mutex::new(pool)) }
        }
    }

    impl PoolManager for PostgresPoolManager {
        type Connection = TConnection;
        fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<TConnection>>>> {
            &self.pool
        }
    }

    impl From<PostgresPoolManager> for ConnectionDriver {
        fn from(val: PostgresPoolManager) -> Self {
            ConnectionDriver::Postgres(val)
        }
    }
}

#[async_trait]
pub trait PoolManager {
    type Connection: diesel::r2d2::R2D2Connection + 'static;
    fn get_pool(&self) -> &Option<Mutex<Pool<ConnectionManager<Self::Connection>>>>;

    fn get(&self) -> diesel::r2d2::PooledConnection<ConnectionManager<Self::Connection>> {
        let binding = self.get_pool().as_ref().unwrap();
        let pool = binding.lock().unwrap();
        pool.get().unwrap()
    }

    async fn query<F, R>(&self, f: F) -> AppResult<R>
    where
        F: FnOnce(diesel::r2d2::PooledConnection<ConnectionManager<Self::Connection>>) -> QueryResult<R> + Send + 'static,
        R: Send + 'static,
    {
        let conn = self.get();

        let result = task::spawn_blocking(move || f(conn)).await?;

        if let Err(e) = result {
            return Err(nidrs::AppError::Exception(nidrs::Exception::new(http::StatusCode::INTERNAL_SERVER_ERROR, anyhow::Error::new(e))));
        }

        Ok(result.unwrap())
    }
}
