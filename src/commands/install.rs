extern crate yaml_rust;

use dialoguer::MultiSelect;
use std::fs;
use std::fs::File;
use std::io;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const LIB_PATH: &str = "/usr/local/bin";

#[cfg(any(target_os = "windows"))]
const LIB_PATH: &str = "C://ProgramData/mgpm";

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
    let packages = crate::packagelist::import_packagelist();
    let mut choices = vec![];
    let mut defaults = vec![];

    for package in &packages {
        choices.push(package.name.clone());
        defaults.push(false);
    }

    let selections: Vec<usize> = MultiSelect::new()
        .with_prompt("インストールするパッケージを選んでください")
        .items(&choices)
        .defaults(&defaults)
        .interact()?;

    let selected_packages: Vec<_> = selections.iter().map(|&i| &packages[i]).collect();

    for package in selected_packages {
        const OS: &str = std::env::consts::OS;
        const ARCH: &str = std::env::consts::ARCH;
        let url = format!(
            "{}/releases/latest/download/{}-{OS}-{ARCH}",
            package.repository.replace(".git", ""),
            package.name
        );
        println!("{:?}", url);
        let filename = format!("{LIB_PATH}/{}", package.name);
        let response: reqwest::Response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        let mut out = File::create(filename.clone())?;
        io::copy(&mut bytes.as_ref(), &mut out)?;

        let _ = set_execute_permissions(filename);
    }

    Ok(())
}
