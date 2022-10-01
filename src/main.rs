/*
 * the idea is to test google tasks and try to
 * get - todo items
 * post - ...
 * patch - (easy on network) - also RESTfull
 * delete - ...
 *
 */

use reqwest::{get, Error};
use serde::Deserialize;

// also we have static as mutable const with 'static lifetime
const USER_API: &str = "https://mocki.io/v1/d4867d8b-b5d5-4a48-a4ab-79131b5809b8";

// serde is the beast! )
#[derive(Deserialize, Debug)]
struct User {
    name: String,
    city: String,
}

// tokio is the most conventient method I know yet for async main fn
#[tokio::main]
async fn main() -> Result<(), Error> {
    let res: Vec<User> = get(USER_API).await?.json().await.expect("no json");
    println!("{:#?}", res);

    Ok(())
}
