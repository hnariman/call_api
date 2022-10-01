use reqwest::{Error, get};
use serde::Deserialize;

static USER_API: &str = "https://mocki.io/v1/d4867d8b-b5d5-4a48-a4ab-79131b5809b8";

#[derive(Deserialize, Debug)]
struct User {
    name: String,
    city: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let res:Vec<User> = get(USER_API).await?.json().await.expect("no json");
    println!("{:#?}", res);

    Ok(())
}
