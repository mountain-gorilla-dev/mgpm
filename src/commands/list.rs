use std::fs;

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
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
}
