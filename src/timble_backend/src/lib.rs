use std::os::unix::raw::gid_t;

use candid::CandidType;
use ic_sqlite::CONN;
use serde::{Deserialize, Serialize};
#[macro_use]
extern crate ic_cdk_macros;

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[derive(Deserialize, Debug)]
struct GeocodeResponse {
    lat: String,
    lon: String,
}

// async fn get_location(city: &str) -> Result {
//     let url = format!("https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",city);

//     let rx = reqwest::get(&url).await?;

//     let p:Vec<GeocodeResponse> =rx.json().await?;
//     if let Some(entry) = 
//     Result<_, Err); 
// }

// async fn fetch_coordinates(location: &str) -> Result<(f64, f64), Error> {
//     let url = format!(
//         "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
//         location
//     );

//     let response = reqwest::get(&url).await.map_err(|e| Error::CanisterError {
//         message: format!("Failed to call geocoding API: {}", e),
//     })?;

//     let data: Vec<serde_json::Value> = response.json().await.map_err(|e| Error::CanisterError {
//         message: format!("Failed to parse geocoding API response: {}", e),
//     })?;

//     if let Some(entry) = data.get(0) {
//         let lat = entry["lat"]
//             .as_str()
//             .ok_or_else(|| Error::CanisterError {
//                 message: "Latitude not found in API response".to_string(),
//             })?
//             .parse::<f64>()
//             .map_err(|e| Error::CanisterError {
//                 message: format!("Failed to parse latitude: {}", e),
//             })?;

//         let lon = entry["lon"]
//             .as_str()
//             .ok_or_else(|| Error::CanisterError {
//                 message: "Longitude not found in API response".to_string(),
//             })?
//             .parse::<f64>()
//             .map_err(|e| Error::CanisterError {
//                 message: format!("Failed to parse longitude: {}", e),
//             })?;

//         Ok((lat, lon))
//     } else {
//         Err(Error::CanisterError {
//             message: format!("Location '{}' not found", location),
//         })
//     }
// }

#[init]
fn initializer() {
    let connection = CONN.lock().unwrap();

    connection
        .execute(
            "
            CREATE TABLE IF NOT EXISTS User (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL DEFAULT 'Sanjna Singh',
                age INTEGER NOT NULL DEFAULT 21,
                gender TEXT NOT NULL DEFAULT 'Female',
                location TEXT NOT NULL DEFAULT 'Mumbai',
                images_links TEXT NOT NULL DEFAULT '[]'
            );",
            [],
        )
        .expect("Failed to create table");
}

#[derive(CandidType, Deserialize)]
enum Error {
    InvalidCanister,
    CanisterError { message: String },
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
enum Gender {
    Male,
    Female,
}

type Result<T = String, E = Error> = std::result::Result<T, E>;

#[derive(CandidType, Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    age: u16,
    gender: String,
    location: String,
    image_links: Vec<String>
}

#[query]
fn get_all_user() -> Result {
    let connection = CONN.lock().unwrap();

    let mut ex = connection.prepare("SELECT id, name, age, gender, location,images_links FROM User;").map_err(|error| {
        Error::CanisterError { message: format!("Failed to prepare query {}", error) }
    })?;



   let ps =  ex.query_map([], |row| {

    let ts:String = row.get(5)?;
    let images = serde_json::from_str(&ts).unwrap_or_else(|_| vec![]);
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
            gender: row.get(3)?,
            location: row.get(4)?,
            image_links: images
        })
    }).map_err(|error| {
        Error::CanisterError { message: format!("2 Failed to prepare query {}", error) }
    })?;

    let mut res = Vec::new();
    for p in ps {
        match p {
            Ok(u) => res.push(format!(
                "ID: {}, Name: {}, Age: {}, Gender: {}, Location: {}, Images: {:?}",
                u.id, u.name, u.age, u.gender, u.location, u.image_links
            )),
            Err(e) => {
                return Err(Error::CanisterError {
                    message: format!("Failed to map row: {}", e),
                })
            }
        }
    }

    // Join all user strings into a single output
    Ok(res.join("\n"))
}

#[update]
fn create_new_user(name: String, age: u16, gender: Gender, location: String, iamges: Option<Vec<String>>) -> Result {
    let connection = CONN.lock().unwrap();

    // Convert Gender enum to string for database storage
    let gender_str = match gender {
        Gender::Female => "Female",
        Gender::Male => "Male",
    };

    let img = serde_json::to_string(&iamges.unwrap_or_else(|| vec![
            "https://example.com/default1.jpg".to_string(),
            "https://example.com/default2.jpg".to_string(),
            "https://example.com/default3.jpg".to_string(),
            "https://example.com/default4.jpg".to_string(),
            "https://example.com/default5.jpg".to_string(),
            "https://example.com/default6.jpg".to_string(),
    ])).map_err(|e| Error::CanisterError { message: format!("There is error in img {}",e) })?;

    // Insert user into the database
    let result = connection.execute(
        "INSERT INTO User (name, age, gender, location, images_links) VALUES (?1, ?2, ?3, ?4, ?5);",
        (name.clone(), age, gender_str, location,img),
    );

    match result {
        Ok(_) => Ok(format!("User '{}' added successfully!", name)),
        Err(e) => Err(Error::CanisterError {
            message: format!("Failed to add user: {}", e),
        }),
    }
}

ic_cdk::export_candid!();