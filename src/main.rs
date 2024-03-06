use std::fs;
use std::io;

fn main() {
    println!("Decompress File!");
    decompress();
}

fn decompress() -> i32 {
    // Collect command-line arguments into a vector
    let args: Vec<_> = std::env::args().collect();

    // Check if the number of arguments is less than 2
    if args.len() < 2 {
        println!("Usage: {} <fileName>", args[0]);
        return 1;
    }

    // Get the file name from the command-line arguments
    let fName = std::path::Path::new(&*args[1]);

    // Open the specified file for reading
    let file = fs::File::open(&fName).unwrap();

    // Create a ZipArchive from the opened file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // Iterate over each file in the zip archive
    for i in 0..archive.len() {
        // Get the i-th file from the archive
        let mut file = archive.by_index(i).unwrap();

        // Get the enclosed name of the file
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Print the file comment if it exists
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        // Check if the file ends with '/'
        if (*file.name()).ends_with('/') {
            // Create directories for the file path
            println!("File {} extracted to {}", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // Extract and write the file contents
            println!(
                "File {} extracted to {} ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            // Create parent directories if they don't exist
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            // Create and open the output file
            let mut outfile = fs::File::create(&outpath).unwrap();

            // Copy the file contents to the output file
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Set file permissions (Unix-specific)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // Return a status code
    return 1;
}
