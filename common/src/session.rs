use tokio_postgres::Error;

use crate::Database;

impl Database {
    pub fn add_session(session: String) -> Result<(), Error>{

        Ok(())
    }

    pub fn verify_session(session: String)  -> Result<bool, Error>{
        Ok(true)
    }
}