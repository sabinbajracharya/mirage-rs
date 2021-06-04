
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use diesel::sqlite::SqliteConnection;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

embed_migrations!();

lazy_static! {
  static ref POOL: Pool = {
      let db_url = env::var("DATABASE_URL").expect("Database url not set");
      let manager = ConnectionManager::<SqliteConnection>::new(db_url);
      Pool::new(manager).expect("Failed to create db pool")
  };
}

pub fn init() {
  lazy_static::initialize(&POOL);
  let conn = connection().expect("Failed to get db connection");
  embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, String> {
  POOL.get()
    .map_err(|e| format!("Failed getting db connection: {}", e))
}