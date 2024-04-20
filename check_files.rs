use std::process::{Command, Stdio};
use std::fs;
use std::io::Write;

// Function to run Dafny verification on a list of files
fn run_dafny_verification(file_paths: Vec<String>, start_index: usize) -> Vec<(String, String)> {
    let mut failed_files = vec![];
    for (index, file_path) in file_paths.iter().enumerate() {
        let mut command = Command::new("dafny");
        command.args(&["verify", file_path, "--verification-time-limit", "60", "--cores", "7"]);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        match command.output() {
            Ok(output) => {
                if !output.status.success() {
                    let error_message = String::from_utf8_lossy(&output.stderr).to_string();
                    failed_files.push((file_path.clone(), error_message));
                }
                println!("Verification attempt {} for file: {}", start_index + index + 1, file_path);
            },
            Err(err) => {
                println!("Error occurred: {}", err);
            }
        }
        
        // Clear memory
        // No equivalent for gc.collect() in Rust, memory management is handled automatically
    }
    failed_files
}

fn main() {
    // Directory containing Dafny files
    let directory = "./compilable";

    // List to store file paths
    let mut file_paths = Vec::new();

    // Iterate through files in the directory and sort alphabetically
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".dfy") {
                        file_paths.push(entry.path().to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
    file_paths.sort(); // Sort alphabetically

    // Break files into chunks of 100
    let chunk_size = 100;
    let file_chunks: Vec<_> = file_paths.chunks(chunk_size).collect();

    // Process each chunk sequentially
    let mut start_index = 1;
    for (chunk_index, chunk) in file_chunks.iter().enumerate() {
        println!("Processing chunk {}...", chunk_index + 1);
        let failed_files = run_dafny_verification(chunk.to_vec(), start_index);

        // Write failed verifications to output file
        let output_file_path = format!("failed_files_chunk_{}.txt", chunk_index + 1);
        let mut output_file = fs::File::create(output_file_path).expect("Failed to create output file");
        for (failed_file, error_message) in failed_files {
            writeln!(&mut output_file, "File '{}' failed verification with error: {}", failed_file, error_message).expect("Failed to write to output file");
        }
        
        start_index += chunk.len();
    }
}
