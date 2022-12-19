use dotenv::dotenv;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    dotenv().ok();
    let (_, session) = env::vars()
        .find(|(key, _)| key == "SESSION")
        .expect("for session id to be in an environment variable");
    let arg = env::args().nth(1).expect("Incorrect number of arguments");
    let day = arg
        .parse::<u32>()
        .expect("Invalid argument. Please enter a valid day");
    let client = reqwest::Client::new();
    let input = client
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header(reqwest::header::COOKIE, format!("session={}", session))
        .send()
        .await?
        .text()
        .await?;
    fs::write("input.txt", input).expect("unable to write to input.txt file");
    Ok(())
}
