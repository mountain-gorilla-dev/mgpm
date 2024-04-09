extern crate yaml_rust;

use dialoguer::MultiSelect;
use std::fs;
use std::fs::File;
use std::io;
use yaml_rust::YamlLoader;

fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();

    YamlLoader::load_from_str(&s).unwrap()
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn set_execute_permissions(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(filename.clone())?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(filename.clone(), perms)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_execute_permissions(filename: String) {}

#[tokio::main]
pub async fn install(_: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let path = "./packagelist.yml";
    let docs = load_yaml(path);
    let doc = &docs[0];
    let mut choices = vec![];
    let mut defaults = vec![];

    for package in doc["packages"].as_hash().unwrap() {
        choices.push(package.1["name"].as_str().unwrap());
        defaults.push(false);
    }

    let selections: Vec<usize> = MultiSelect::new()
        .with_prompt("インストールするパッケージを選んでください")
        .items(&choices)
        .defaults(&defaults)
        .interact()?;

    let vec_packages = doc["packages"]
        .as_hash()
        .unwrap()
        .iter()
        .collect::<Vec<_>>();
    let packages = selections
        .iter()
        .map(|i| vec_packages[*i])
        .collect::<Vec<_>>();

    for package in &packages {
        let repository = package.1["repository"].as_str().unwrap();
        println!("{:?}", repository);
        let url = package.1["bin_url"].as_str().unwrap();
        let filename = format!("./opt/{}", url.split('/').last().unwrap());
        let response: reqwest::Response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        let mut out = File::create(filename.clone())?;
        io::copy(&mut bytes.as_ref(), &mut out)?;

        let _ = set_execute_permissions(filename);
    }

    Ok(())
}
