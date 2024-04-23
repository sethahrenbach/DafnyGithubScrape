use std::process::{Command, Stdio};
use std::io::{Write, Read}; 
use std::fs; 
extern crate wait_timeout;
use wait_timeout::ChildExt;
use std::time::Duration;

fn run_dafny_verification(file_paths: Vec<String>, start_index: usize, chunk_size: usize) -> Vec<(String, String)> {
    let mut failed_files = Vec::new();
    let timeout = Duration::from_secs(400); 

    for file_path in file_paths.iter().skip(start_index).take(chunk_size) {
        println!("Starting verification for: {}", file_path);

        let mut child = Command::new("dafny")
            .args(["verify", file_path, "--cores", "7"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start dafny verification process");

        match child.wait_timeout(timeout).expect("Failed to wait on dafny verification process") {
            Some(status) if status.success() => println!("Verification successful for: {}", file_path),
            Some(_) => {
                let mut stderr = Vec::new();
                child.stderr.as_mut().unwrap().read_to_end(&mut stderr).unwrap(); // Properly use read_to_end
                failed_files.push((file_path.clone(), String::from_utf8_lossy(&stderr).to_string()));
                println!("Process failed: {}", String::from_utf8_lossy(&stderr));
            },
            None => {
                child.kill().expect("Failed to kill the process");
                child.wait().expect("Failed to wait for the process to terminate");
                failed_files.push((file_path.clone(), String::from("Process killed due to timeout")));
            }
        }
    }

    failed_files
}

fn main() {
    let directory = "../compilable";
    let mut file_paths = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".dfy") {
                    file_paths.push(entry.path().to_str().unwrap().to_string());
                }
            }
        }
    }

    // Sort the file paths alphabetically
    file_paths.sort();

    let chunk_size = 20;

    for start_index in (0..file_paths.len()).step_by(chunk_size) {
        let failed_files = run_dafny_verification(file_paths.clone(), start_index, chunk_size);

        let output_file_path = format!("failed_files_chunk_{}.txt", start_index / chunk_size + 1);
        let mut output_file = fs::File::create(&output_file_path).expect("Failed to create output file");
        for (failed_file, error_message) in failed_files {
            writeln!(output_file, "File '{}' failed verification with error: {}", failed_file, error_message).expect("Failed to write to output file");
        }
    }
}
