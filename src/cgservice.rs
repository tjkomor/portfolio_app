use isahc::prelude::*;

pub fn get_token(token: &str) -> Result<(), isahc::Error> {
    let url = format!("https://api.coingecko.com/api/v3/coins/{}", token);
    println!("{}", &url);
    let mut response = isahc::get(url)?;

    // Print some basic info about the response to standard output.
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
    print!("{}", response.text()?);

    Ok(())
}

pub fn get_all_currencies() -> Result<(), isahc::Error> {
    let url = "https://api.coingecko.com/api/v3/simple/supported_vs_currencies";
    println!("{}", &url);
    let mut response = isahc::get(url)?;

    // Print some basic info about the response to standard output.
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
    print!("{}", response.text()?);

    Ok(())
}

