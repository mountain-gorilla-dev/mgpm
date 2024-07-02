use std::ffi::OsString;
use std::fs;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const LIB_PATH: &str = "/usr/local/bin";

#[cfg(any(target_os = "windows"))]
const LIB_PATH: &str = "C://ProgramData/mgpm";

pub fn remove(name: &Option<String>, all: bool) -> Result<(), Box<dyn std::error::Error>> {
    // let entries = fs::read_dir(LIB_PATH).unwrap(); // ReadDir を取得
    let packages = crate::packagelist::import_packagelist();

    if all {
        println!("本当に削除してよろしいですか？ (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            for package in packages {
                remove_file(&package.path)?;
            }
        }
        return Ok(());
    }

    if name.is_none() {
        println!("パッケージ名を指定してください\n例：mgpm remove 〇〇");
        return Ok(());
    }

    let str_name = name.clone().unwrap().to_string();

    if packages.into_iter().any(|p| p.name == str_name) {
        let path = format!("{LIB_PATH}/{}", name.clone().unwrap());
        remove_file(&path)?;
        println!("{:?} を削除しました", str_name);
        return Ok(());
    } else {
        println!("{:?} はインストールされていません", str_name);
    }
    Ok(())
}

fn remove_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = match fs::metadata(path) {
        Ok(_) => fs::remove_file(path),
        // Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    };

    Ok(())
}
