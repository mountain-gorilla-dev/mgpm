use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SupportArch {
    pub x86_64: bool,
    pub aarch64: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Supported {
    pub linux: SupportArch,
    pub mac: SupportArch,
    pub windows: SupportArch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YAMLPackage {
    pub name: String,
    pub description: String,
    pub repository: String,
    supported: Supported,
}

pub struct Package {
    pub name: String,
    pub description: String,
    pub repository: String,
    supported: Supported,
    pub path: String,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
const LIB_PATH: &str = "/usr/local/bin";

#[cfg(any(target_os = "windows"))]
const LIB_PATH: &str = "C://ProgramData/mgpm";

pub fn import_packagelist() -> Vec<Package> {
    let yaml_packages: Vec<YAMLPackage> =
        serde_yaml::from_str(include_str!("../packagelist.yml")).unwrap();
    // let packages: Vec<Package> = yaml_packages.iter().map(|p| p.path = )
    let mut packages: Vec<Package> = vec![];
    for yaml_package in yaml_packages {
        let path = format!("{LIB_PATH}/{}", yaml_package.name);
        packages.push(Package {
            name: yaml_package.name,
            description: yaml_package.description,
            repository: yaml_package.repository,
            supported: yaml_package.supported,
            path,
        })
    }
    packages
}
