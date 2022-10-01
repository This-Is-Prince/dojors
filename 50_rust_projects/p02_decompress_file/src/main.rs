extern crate zip;

use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use zip::ZipArchive;

fn main() {
    std::process::exit(real_main())
}

fn real_main() -> i32 {
    println!("Decompress file");
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let file_path = Path::new(&*args[1]);
    let file = File::open(&file_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let out_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if file.name().ends_with('/') {
            println!("File {} extracted to \"{}\"", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({:.2} kb)",
                i,
                out_path.display(),
                (file.size() as f64 / 1024.)
            );

            if let Some(p) = out_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut out_file = File::create(&out_path).unwrap();
            io::copy(&mut file, &mut out_file).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode)=file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}
