use reqwest::{header::HeaderValue, StatusCode};
use std::{fs, path::Path};

pub mod solution;

const YEAR: &str = "2021";
const DAY: &str = "12";

pub fn fetch_input() -> String {
    let input_path = Path::new("input.txt");
    if !input_path.exists() {
        let session_token_path_name =
            format!("{}/aoc/.session-token", std::env::var("HOME").unwrap());
        let session_token = fs::read_to_string(Path::new(session_token_path_name.as_str()))
            .expect("Cannot read session token");
        let session_cookie = ["session=", &session_token.trim()].concat();
        let client = reqwest::blocking::Client::new();
        let request = client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                YEAR, DAY
            ))
            .header(
                "cookie",
                HeaderValue::from_str(&session_cookie)
                    .expect("Unable to build session cookie header value"),
            )
            .build()
            .expect("Cannot build request");

        let input_response = client.execute(request).expect("Cannot fetch input");
        if input_response.status() != StatusCode::OK {
            println!("{:?}", input_response);
            panic!("Did not get OK when fetching input");
        }
        let input = input_response.text().expect("Cannot get input text");

        fs::write(input_path, input).expect("Cannot write input to file");
    }
    fs::read_to_string(input_path).expect("Cannot read input file")
}
