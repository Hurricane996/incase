use tokio_postgres::Client;

mod video;

pub struct Database {
    client: Client,
}



