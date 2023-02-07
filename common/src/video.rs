use tokio::try_join;
use tokio_postgres::{types::ToSql, NoTls};

use crate::Database;

impl Database {
    pub async fn new(connection_string: &str) -> Result<Self, tokio_postgres::Error> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e)
            }
        });

        Ok(Self {
            client
        })
    
    }

    pub async fn clear_video(&mut self, user_id: &str) -> Result<(), tokio_postgres::Error> {
        const SQL: &str = "DELETE FROM video_segment WHERE user_id = $1";

        self.client.execute(SQL, &[&user_id]).await?;

        Ok(())
    }

    pub async fn get_video_clip(&mut self, user_id: &str, sequence_number: i32) -> Result<Vec<u8>,tokio_postgres::Error> {
        const SQL: &str = "SELECT data FROM video_segments WHERE user_id = $1 AND sequence_number = $2";

        let data = self.client.query_one(SQL, &[&user_id, &sequence_number]).await?;

        Ok(data.get("data"))

    }

    pub async fn get_sequence_numbers(&mut self, user_id: &str) -> Result<Vec<i32>,tokio_postgres::Error> {
        const SQL: &str = "SELECT sequence_number FROM video_segments WHERE user_id = $1;";
        
        let nums = self.client.query(SQL, &[&user_id]).await?;

        Ok(nums.iter().map(|x| x.get("sequence_number")).collect())
    }

    pub async fn post_video_clip(
        &mut self,
        user_id: &str,
        data: &[u8]
    ) -> Result<(),tokio_postgres::Error>{
        const MAIN_SQL: &'static str = "
INSERT INTO video_segments (sequence_number, user_id, data)
VALUES ((SELECT COALESCE(1+ MAX(sequence_number),0) FROM video_segments WHERE user_id=$1), $1, $2);";

        const CLEANUP_SQL: &'static str = "
DELETE FROM video_segments WHERE user_id = $1 AND 
sequence_number <= (SELECT COALESCE(MAX(sequence_number) - 1024, 0) FROM video_segments WHERE user_id = $1);";
        let transaction = self.client.transaction().await?;

        let main_args: [&(dyn ToSql + Sync);2] = [&user_id, &data];

        let cleanup_args: [&(dyn ToSql + Sync); 1] = [&user_id];

        try_join!(
            transaction.execute(MAIN_SQL, &main_args),
            transaction.execute(CLEANUP_SQL, &cleanup_args)
        )?;

        transaction.commit().await?;
        Ok(())
    }
}