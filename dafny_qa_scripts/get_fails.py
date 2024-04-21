import os

def aggregate_lines():
    total_line_count = 0
    output_file_path = 'aggregated_lines.txt'
    
    # Open the output file for writing
    with open(output_file_path, 'w') as output_file:
        # Loop through all file numbers and construct each filename
        for i in range(1, 56):  # From 1 to 55
            file_name = f'failed_files_chunk_{i}.txt'
            
            # Check if the file exists
            if os.path.exists(file_name):
                # Open the file and read lines
                with open(file_name, 'r') as file:
                    for line in file:
                        # Strip whitespace and check if line is not empty
                        if line.strip():
                            # Write the non-empty line to the output file
                            output_file.write(line)
                            total_line_count += 1
            else:
                print(f"File {file_name} does not exist.")

    print(f"Total number of non-empty lines: {total_line_count}")

def main():
    aggregate_lines()

if __name__ == "__main__":
    main()
