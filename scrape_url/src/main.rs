

use std::fs;
// main 函数现在返回一个 Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}

fn fib_loop(n: u8) {
  let (mut a, mut b) = (1, 1);

  for _i in 2..n {
    let c = a + b;
    a = b;
    b = c;
    println!("next value is {}", b);
  }
}