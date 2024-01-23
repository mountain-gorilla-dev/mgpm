use std::collections::HashMap;


use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[arg(value_name = "URL", help = "リクエストしたいURLを指定してください", default_value = "")]
    url: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::parse();

  if args.url == "" {
    println!("引数を指定してください");
    return Ok(())
  }

  println!("{}", args.url);

  let resp = reqwest::get(args.url)
      .await?
      .json::<HashMap<String, String>>()
      .await?;
  println!("{:#?}", resp);
  Ok(())
}
