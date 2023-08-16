use std::env;
use std::process::exit;
use diesel::connection::SimpleConnection;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager};

pub fn establish_connection() -> r2d2::Pool<ConnectionManager<PgConnection>> {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    if let Ok(pool) = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
    {
        info!("PGSQL connection pool established.");
        pool.get().unwrap().batch_execute("SET application_name TO 'phishing-api';").unwrap();
        pool
    } else {
        error!("Failed to create pool, please check your database connection string.");
        exit(1);
    }
}
