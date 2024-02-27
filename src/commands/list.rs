extern crate yaml_rust;

use dialoguer::MultiSelect;
use std::fs;
use std::fs::File;
use std::io;
use std::os::unix::fs::PermissionsExt;
use yaml_rust::YamlLoader;

fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}

#[tokio::main]
pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir("./opt").unwrap(); // ReadDir を取得

    // ループで Result<DieEntry, Error> をひとつずつ処理
    for entry in entries {
        // DirEntry#file_name() でファイル名（ディレクトリ名）を取得できる
        let file_name = entry.unwrap().file_name();
        if file_name != ".gitignore" {
            println!("{:?}", file_name);
        }
    }
    Ok(())
    // let path = "./packagelist.yml";
    // let docs = load_yaml(&path);
    // let doc = &docs[0];
    // let mut choices = vec![];
    // let mut defaults = vec![];

    // for package in doc["packages"].as_hash().unwrap() {
    //     choices.push(package.1["name"].as_str().unwrap());
    //     defaults.push(false);
    //     // println!("{:?}", package.1["name"].as_str().unwrap());
    //     // println!("{:?}", package);
    // }

    // let selections: Vec<usize> = MultiSelect::new()
    //     .with_prompt("インストールするパッケージを選んでください")
    //     .items(&choices)
    //     .defaults(&defaults)
    //     .interact()?;

    // let vec_packages = doc["packages"]
    //     .as_hash()
    //     .unwrap()
    //     .iter()
    //     .collect::<Vec<_>>();
    // let packages = selections
    //     .iter()
    //     .map(|i| vec_packages[*i])
    //     .collect::<Vec<_>>();

    // for package in &packages {
    //     let repository = package.1["repository"].as_str().unwrap();
    //     println!("{:?}", repository);
    //     let url = package.1["bin_url"].as_str().unwrap();
    //     let filename = format!("./opt/{}", url.split("/").last().unwrap());
    //     let response: reqwest::Response = reqwest::get(url).await?;
    //     let bytes = response.bytes().await?;
    //     let mut out = File::create(filename.clone())?;
    //     io::copy(&mut bytes.as_ref(), &mut out)?;

    //     let mut perms = fs::metadata(filename.clone())?.permissions();
    //     perms.set_mode(0o755);
    //     fs::set_permissions(filename.clone(), perms)?;
    // }

    // Ok(())
}
