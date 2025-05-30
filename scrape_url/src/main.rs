use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut url = "https://www.rust-lang.org/";
    let mut output = "rust.md";
    if args.len() < 3 {
        panic!("args is len than 3, Run: scrape_url url outputName");
    }

    if let [_cmd, u, o, ..] = args.as_slice() {
        (url, output) = (u, o);
        println!("Fetching url: {}, Output: {}", url, output);
    } else {
        eprintln!("args is short");
    }

    let body = reqwest::blocking::get(url)?.text()?;

    println!("Coverting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}", output);
    Ok(())
}
