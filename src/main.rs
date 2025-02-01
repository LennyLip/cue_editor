use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use encoding_rs::WINDOWS_1252; // Import encoding_rs

fn main() -> io::Result<()> {
    // Get the current directory
    let current_dir = std::env::current_dir()?;
    println!("Searching for .cue files in: {}", current_dir.display());
    // Recursively search for .cue files
    find_and_process_cue_files(&current_dir)?;
    Ok(())
}

fn find_and_process_cue_files(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Recursively process subdirectories
                find_and_process_cue_files(&path)?;
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("cue") {
                // If it's a .cue file, process it
                if let Err(e) = process_cue_file(&path) {
                    eprintln!("Failed to process file {}: {}", path.display(), e);
                }
            }
        }
    }
    Ok(())
}

fn process_cue_file(cue_path: &Path) -> io::Result<()> {
    // Read the file as raw bytes
    let content_bytes = fs::read(cue_path)?;

    // Attempt to decode the file using UTF-8 or fallback to Windows-1252
    let (content, encoding_used) = decode_with_fallback(&content_bytes);

    // Log the detected encoding
    println!(
        "File {} was decoded using {}",
        cue_path.display(),
        encoding_used
    );

    let mut new_content = content.clone();

    // Get the list of files in the same directory
    let dir = cue_path.parent().unwrap();
    let files_in_dir: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Check for the presence of .flac or .ape files
    let has_flac = files_in_dir.iter().any(|file| file.extension().and_then(|ext| ext.to_str()) == Some("flac"));
    let has_ape = files_in_dir.iter().any(|file| file.extension().and_then(|ext| ext.to_str()) == Some("ape"));

    if has_flac {
        // Replace ".wav" with ".flac"
        new_content = new_content.replace(".wav", ".flac");
        println!(
            "Replaced '.wav' with '.flac' in file: {}",
            cue_path.display()
        );
    } else if has_ape {
        // Replace ".wav" with ".ape"
        new_content = new_content.replace(".wav", ".ape");
        println!(
            "Replaced '.wav' with '.ape' in file: {}",
            cue_path.display()
        );
    }

    // If the content has changed, overwrite the file
    if new_content != content {
        let mut file = fs::File::create(cue_path)?;
        file.write_all(new_content.as_bytes())?;
    }

    Ok(())
}

/// Decode the file content using UTF-8 or fallback to Windows-1252
fn decode_with_fallback(content_bytes: &[u8]) -> (String, &'static str) {
    // Try decoding as UTF-8
    if let Ok(valid_utf8) = String::from_utf8(content_bytes.to_vec()) {
        return (valid_utf8, "UTF-8");
    }

    // Fallback to Windows-1252
    let (decoded, _, _) = WINDOWS_1252.decode(content_bytes);
    (decoded.into_owned(), "Windows-1252")
}