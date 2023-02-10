use std::{error::Error, sync::Arc};

use common::Database;
use tokio::{net::TcpListener, io::AsyncReadExt};
const CHUNK_SIZE: usize = 1024 * 128;
#[tokio::main]

async fn main() -> Result<(),Box<dyn Error>>{
    let connection_string = Arc::new(std::env::var("POSTGRES_CONNECTION_STRING").expect("No postgres connection string provided"));

    let listener = TcpListener::bind("0.0.0.0:1234").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await?;
        let connection_string = connection_string.clone();
        tokio::spawn(async move {
            let mut database = Database::new(&connection_string).await.unwrap();
            // todo grab this from token
            let user_id = "auth0|63e0ad87641e1d30b85c7282";
            'outer : loop {
                let mut offset = 0;
                let mut buf = [0;CHUNK_SIZE];

                'inner: loop {
                    let n = match socket.read(&mut buf[offset..CHUNK_SIZE]).await {
                        Ok(n) => n,
                        Err(_) => 0,
                    };

                    offset += n;
                    if offset >= CHUNK_SIZE || n == 0 {
                        database.post_video_clip(user_id, &buf).await.unwrap();
                    }

                    if offset >= CHUNK_SIZE { break 'inner; }
                    if n == 0 { break 'outer; }
                }
            }
            Result::<(), tokio_postgres::Error>::Ok(())
        });
    } 
}

