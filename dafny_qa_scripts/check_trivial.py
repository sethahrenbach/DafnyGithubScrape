import os

# Directory path
current_directory = os.getcwd()
parent_directory = os.path.dirname(current_directory)
compilable_directory = os.path.join(parent_directory, "compilable")

# Output file path
output_file = os.path.join(current_directory + '/trivial_files.txt')
output_file2 = os.path.join(current_directory + '/trivial_files2.txt')

# Open the output file in append mode
with open(output_file, 'a') as file:
    # Iterate through the files in the directory
    for filename in os.listdir(compilable_directory):
        # Check if the file is a regular file
        if os.path.isfile(os.path.join(compilable_directory, filename)):
            # Read the contents of the file
            with open(os.path.join(compilable_directory, filename), 'r') as f:
                contents = f.read()
            
            # Check if the file contains the word "ensures"
            if 'ensures' not in contents:
                if 'requires' not in contents:
                    # Write the filename to the output file
                    file.write(filename + '\n')
            
# Open the output file in append mode
with open(output_file2, 'a') as file:
    # Iterate through the files in the directory
    for filename in os.listdir(compilable_directory):
        # Check if the file is a regular file
        if os.path.isfile(os.path.join(compilable_directory, filename)):
            # Read the contents of the file
            with open(os.path.join(compilable_directory, filename), 'r') as f:
                contents = f.read()
            
                        
            # Check if the file contains the word "invariant" or "assert"
            if 'invariant' not in contents:
                if 'assert' not in contents:
                    # Write the filename to the output file
                    file.write(filename + '\n')