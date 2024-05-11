import os

# Get the current directory
current_directory = os.getcwd()

# Get the parent directory
parent_directory = os.path.dirname(current_directory)

# Define the path to the trivial_files.txt file
trivial_files_path = os.path.join(current_directory, "trivial_files.txt")

# Iterate through lines in trivial_files.txt
with open(trivial_files_path, "r") as file:
    for line in file:
        # Remove leading/trailing whitespaces and newline characters
        line = line.strip()

        # Construct the path to the file in the parent directory/compilable/
        file_path = os.path.join(parent_directory, "compilable", line)

        # Check if the file exists
        if os.path.exists(file_path):
            # Remove the file
            os.remove(file_path)
            print(f"Removed file: {file_path}")
        else:
            print(f"File not found: {file_path}")