use std::ffi::OsString;
use std::fs;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const LIB_PATH: &str = "/usr/local/bin";

#[cfg(any(target_os = "windows"))]
const LIB_PATH: &str = "C://ProgramData/mgpm";

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let packages = crate::packagelist::import_packagelist();

    for package in packages {
        if fs::metadata(package.path).is_ok() {
            println!("{:?}", package.name)
        }
    }
    Ok(())
}
