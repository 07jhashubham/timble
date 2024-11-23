use ic_cdk::update;
use ic_sqlite_features::{Connection, CONN};

#[update]
pub fn create_new_user(name: String, age:u16, prof: String, location: String, interest: Vec<String>, about: String, image: Vec<String> ) -> Result<(), String> {

    let connection = CONN.lock().map_err(|e| e.to_string())?;

    let res = connection.execute(
                "CREATE TABLE IF NOT EXISTS User (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                age INTEGER NOT NULL,
                prof TEXT NOT NULL,
                location TEXT NOT NULL,
                interest TEXT,
                about TEXT NOT NULL,
                image TEXT
            )", []).map_err(|e|e.to_string())?;

    Ok(())
}

fn table_exist(table_name: String) -> Result<bool, String> {
    let connect = CONN.lock().map_err(|e| e.to_string())?;
     connect.prepare("SELECT name FROM sqlite_master where type='table' and name='{}'")
}