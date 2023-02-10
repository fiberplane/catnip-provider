use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    #[serde(rename = "geo")]
    pub geocode: GeoLocation,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct GeoLocation {
    #[serde(deserialize_with = "deserialize_number_from_string", rename = "lat")]
    pub latitude: f64,
    #[serde(deserialize_with = "deserialize_number_from_string", rename = "lng")]
    pub longitude: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub name: String,
    pub catch_phrase: String,
    pub bs: String,
}

pub fn distance_between(origin: GeoLocation, destination: GeoLocation) -> f64 {
    ((destination.longitude - origin.longitude).powi(2)
        + (destination.latitude - origin.latitude).powi(2))
    .sqrt()
}

pub fn closest_user(target: GeoLocation, users: &[User]) -> Option<(f64, User)> {
    users
        .iter()
        .map(|user| (distance_between(user.address.geocode, target), user.clone()))
        .min_by(|(distance_l, _), (distance_r, _)| {
            distance_l
                .partial_cmp(distance_r)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

#[test]
fn test_deserialization() {
    let sample = r#"
{
    "id": 1,
    "name": "Leanne Graham",
    "username": "Bret",
    "email": "Sincere@april.biz",
    "address": {
      "street": "Kulas Light",
      "suite": "Apt. 556",
      "city": "Gwenborough",
      "zipcode": "92998-3874",
      "geo": {
        "lat": "-37.3159",
        "lng": "81.1496"
      }
    },
    "phone": "1-770-736-8031 x56442",
    "website": "hildegard.org",
    "company": {
      "name": "Romaguera-Crona",
      "catchPhrase": "Multi-layered client-server neural-net",
      "bs": "harness real-time e-markets"
    }
  }"#;

    let _ = serde_json::from_str::<User>(sample).unwrap();
}
