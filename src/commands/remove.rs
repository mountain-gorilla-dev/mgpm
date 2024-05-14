use std::ffi::OsString;
use std::fs;

#[cfg(any(target_os = "linux", target_os = "macos"))]
const LIB_PATH: &str = "/var/lib/mgpm";

#[cfg(any(target_os = "windows"))]
const LIB_PATH: &str = "C://ProgramData/mgpm";

pub fn remove(name: &Option<String>, all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir(format!("{LIB_PATH}/opt")).unwrap(); // ReadDir を取得
    let mut pkgs: Vec<OsString> = vec![];
    // ループで Result<DieEntry, Error> をひとつずつ処理
    for entry in entries {
        // DirEntry#file_name() でファイル名（ディレクトリ名）を取得できる
        let file_name = entry.unwrap().file_name();
        if file_name != ".gitignore" {
            // println!("{:?}", file_name);
            pkgs.push(file_name);
        }
    }

    if all {
        println!("本当に削除してよろしいですか？ (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            for pkg in pkgs {
                let path = format!("{LIB_PATH}/opt/{}", pkg.into_string().unwrap());
                fs::remove_file(path)?;
            }
        }
        return Ok(());
    }

    if name.is_none() {
        println!("パッケージ名を指定してください\n例：mgpm remove 〇〇");
        return Ok(());
    }

    let str_name = name.clone().unwrap().to_string();
    if pkgs
        .into_iter()
        .any(|p| p.into_string().unwrap() == str_name)
    {

        let path = format!("{LIB_PATH}/opt/{}", name.clone().unwrap());
        fs::remove_file(path)?;
        println!("{:?} を削除しました", str_name);
        return Ok(());
    } else {
        println!("{:?} はインストールされていません", str_name);
    }
    Ok(())
}
