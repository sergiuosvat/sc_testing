#!/bin/bash

# Define the regex pattern for test names
PREFIX="test"

# Define the output file
OUTPUT_FILE="output.txt"

# Clear the output file
> "$OUTPUT_FILE"

# Get the list of test functions that start with the prefix and clean up names
test_list=$(cargo test -- --list | grep -E "^$PREFIX" | awk -F':' '{print $1}')

# Check if there are any tests to run
if [ -z "$test_list" ]; then
    echo "No tests found with the prefix '$PREFIX'."
    exit 0
fi

# Run each test separately
while IFS= read -r test_name; do
    # Strip any leading/trailing whitespace
    test_name=$(echo "$test_name" | xargs)

    # Log the test name and a separator line
    {
        echo "===================================="
        echo "Running test: $test_name"
        echo "===================================="
        
        # Run the test and capture its output
        cargo test "$test_name" -- --nocapture 2>&1
    } >> "$OUTPUT_FILE"

    # Add an empty line for readability between test outputs
    echo "" >> "$OUTPUT_FILE"
done <<< "$test_list"

echo "All tests have been run. Check $OUTPUT_FILE for details."
