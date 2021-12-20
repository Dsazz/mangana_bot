use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel_migrations::embed_migrations;
embed_migrations!("migrations/");


type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    static ref POOL: Pool = {
        let database_url = database_url();
        let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

        r2d2::Pool::builder()
            .max_size(9)
            .build(manager)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    };
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    POOL.get().unwrap()
}

pub fn run_db_migrations() {
    let connection = &establish_connection();
    embedded_migrations::run(connection).expect("Migration not possible to run");
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}