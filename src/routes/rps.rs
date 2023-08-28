use axum::{http::StatusCode, Json};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RPS {
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct RpsResponse {
    status: String,
    value: String,
    color: String,
}

// impl RpsResponse {
//     pub fn default(self) -> Self {
//       Self { status: String::default(), value: (), color: () }
//     }
// }

pub async fn rps(Json(rps): Json<RPS>) -> Result<Json<RpsResponse>, StatusCode> {
    let vec_of_rps = vec!["rock", "paper", "scissors"];
    let r_p_s = get_random(&vec_of_rps);

    let tie = "It's a tie, try again may be :)";
    let lose = "You lost, let's try again!";
    let win = "You won, congratulations!";

    match (rps.value.as_str(), r_p_s.to_owned()) {
        ("rock", "rock") | ("paper", "paper") | ("scissors", "scissors") => {
            return Ok(Json(RpsResponse {
                status: tie.to_owned(),
                value: String::from(r_p_s.to_owned()),
                color: String::from("#DCF216"),
            }))
        }
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => {
            return Ok(Json(RpsResponse {
                status: win.to_owned(),
                value: String::from(r_p_s.to_owned()),
                color: String::from("#0F9127"),
            }))
        }
        ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => {
            return Ok(Json(RpsResponse {
                status: lose.to_owned(),
                value: String::from(r_p_s.to_owned()),
                color: String::from("#EB3434"),
            }))
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

fn get_random<T>(vector: &[T]) -> &T {
    &vector[rand::thread_rng().gen_range(0..vector.len())]
}
