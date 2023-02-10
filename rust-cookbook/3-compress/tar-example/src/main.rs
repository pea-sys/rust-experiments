use flate2;
use tar;

use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::path::PathBuf;

fn expand() -> Result<(), std::io::Error> {
    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

fn compress() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive2.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "var/log")?;
    Ok(())
}

fn compress_prefix() -> Result<()> {
    let file = File::open("archive3.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "bundle/logs";

    println!("Extracted the following files:");
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf> {
            let path = entry.path()?.strip_prefix(prefix)?.to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| println!("> {}", x.display()));

    Ok(())
}
fn main() {
    match expand() {
        Ok(()) => println!("expand success"),
        Err(e) => println!("expand err value = {}", e),
    }
    match compress() {
        Ok(()) => println!("compress success"),
        Err(e) => println!("compress err value = {}", e),
    }
    match compress_prefix() {
        Ok(()) => println!("compress_prefix success"),
        Err(e) => println!("compress_prefix err value = {}", e),
    }
}
