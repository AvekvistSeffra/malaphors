use elementtree::Element;
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.reddit.com/r/Malaphors/new/.rss?limit=100")
        .await?
        .text()
        .await?;

    let root = match Element::from_reader(resp.as_bytes()) {
        Ok(x) => x,
        Err(err) => panic!("The error was: {:?}", err),
    };

    let tag = "{http://www.w3.org/2005/Atom}entry";

    // let list = match root.find(tag) {
    //     Some(x) => x,
    //     None => panic!("An error finding tag: {:?}", tag),
    // };

    let mut file = File::create("malaphors.txt")?;

    for child in root.find_all(tag) {
        let tag = "{http://www.w3.org/2005/Atom}title";

        if let Some(element) = child.find(tag) {
            file.write_all((element.text().to_owned() + "\n").as_bytes())?;
        }
    }

    Ok(())
}
