use tokio_postgres::Client;

mod video;
mod session;

pub struct Database {
    client: Client,
}



