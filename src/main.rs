// use std::collections::HashMap;

// use clap::Parser;

// /// Search for a pattern in a file and display the lines that contain it.
// #[derive(Parser)]
// struct Cli {
//     #[arg(
//         value_name = "URL",
//         help = "リクエストしたいURLを指定してください",
//         default_value = ""
//     )]
//     url: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args = Cli::parse();

//     if args.url.is_empty() {
//         println!("引数を指定してください");
//         return Ok(());
//     }

//     println!("{}", args.url);

//     let resp = reqwest::get(args.url)
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

extern crate yaml_rust;
use std::fs;
use yaml_rust::{YamlEmitter, YamlLoader};

fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./packagelist.yml";
    let docs = load_yaml(&path);
    let doc = &docs[0];
    // println!("{:?}", doc["packages"].as_str().unwrap());
    // Test
    // assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    // assert_eq!(doc["bar"][0].as_i64().unwrap(), 1);
    // assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);
    assert!(doc["INVALID_KEY"][100].is_badvalue());
    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("{}", out_str);
    Ok(())
}
