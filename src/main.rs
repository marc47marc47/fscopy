use clap::{Arg, Command};
use fs_extra::dir::{self, CopyOptions};
use std::fs;
use std::io::{self, Result};
use std::path::Path;
use std::process::exit;

fn main() -> Result<()> {
    // 定義命令列參數和選項
    let matches = Command::new("Folder Copier")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Copies the contents of one folder to another")
        .arg(
            Arg::new("source")
                .help("Sets the source folder")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("target")
                .help("Sets the target folder")
                .required(true)
                .index(2),
        )
        .get_matches();

    // 取得來源和目標資料夾
    let source_folder = matches.get_one::<String>("source").unwrap();
    let target_folder = matches.get_one::<String>("target").unwrap();

    // 檢查目標資料夾是否存在，不存在則建立
    let target_path = Path::new(target_folder);
    if !target_path.exists() {
        println!("Target folder does not exist, creating it...");
        fs::create_dir_all(target_path)?;
    }

    // 檢查目標資料夾內是否已有檔案存在，詢問是否覆蓋
    if target_path.read_dir()?.next().is_some() {
        println!("Files already exist in the target folder. Do you want to overwrite them? (y/n):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Operation aborted.");
            exit(0);
        }
    }

    // 取得來源資料夾的內容
    let source_path = Path::new(source_folder);
    let entries = fs::read_dir(source_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // 設定複製選項，將目錄的內容複製到目標資料夾
            let mut options = CopyOptions::new();
            options.overwrite = true;
            options.copy_inside = true;

            dir::copy(path, target_folder, &options)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        } else if path.is_file() {
            // 如果是檔案，直接複製
            let file_name = path.file_name().unwrap();
            let target_file_path = target_path.join(file_name);

            fs::copy(&path, target_file_path)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }
    }

    println!("Folder contents copied successfully.");

    Ok(())
}

