import os
import subprocess
import gc

# Function to run Dafny verification on a list of files
def run_dafny_verification(file_paths, start_index):
    failed_files = []
    try:
        for index, file_path in enumerate(file_paths, start=start_index):
            # Run Dafny verification command
            result = subprocess.run(["dafny", "verify", file_path, "--verification-time-limit", "60", "--cores", "7"], capture_output=True, text=True)
            # Check if Dafny verification was successful
            if result.returncode != 0:
                failed_files.append((file_path, result.stderr))
            print(f"Verification attempt {index} for file: {file_path}")
            # Clear memory
            gc.collect()
    except Exception as e:
        print(f"Error occurred: {str(e)}")
    return failed_files

def main():
    # Get the directory of the current Python script
    script_directory = os.path.dirname(os.path.abspath(__file__))
    # Directory containing Dafny files
    directory = os.path.join(script_directory, "compilable")

    # List to store file paths
    file_paths = []

    # Iterate through files in the directory and sort alphabetically
    for filename in sorted(os.listdir(directory)):
        if filename.endswith(".dfy"):
            file_path = os.path.join(directory, filename)
            file_paths.append(file_path)

    # Break files into chunks of 100
    chunk_size = 100
    file_chunks = [file_paths[i:i+chunk_size] for i in range(0, len(file_paths), chunk_size)]

    # Process each chunk sequentially
    start_index = 1
    for chunk_index, chunk in enumerate(file_chunks):
        print(f"Processing chunk {chunk_index + 1}...")
        failed_files = run_dafny_verification(chunk, start_index)

        # Write failed verifications to output file
        with open(f"failed_files_chunk_{chunk_index + 1}.txt", "w") as output_file:
            for failed_file, error_message in failed_files:
                output_file.write(f"File '{failed_file}' failed verification with error: {error_message}\n")
        
        start_index += len(chunk)

if __name__ == "__main__":
    main()
