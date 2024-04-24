#!/bin/bash

echo "Running autogen with --prompt-path ./.prompts/co-2-ch-1/prompt.txt"
# Run Autogen command and capture the output
autogen_output=$(./target/debug/aarya_cli autogen --prompt-path ./.prompts/co-2-ch-1/prompt.txt)

# Check if Autogen command succeeded
if [ $? -eq 0 ]; then
    echo "Autogen command successful"

    # Extract the last line of the output which contains the output file path
    output_file=$(echo "$autogen_output" | tail -n 1)

    echo "Output file path: $output_file"

    # Run Validate command using the output data file path
    ./target/debug/aarya_cli validate --schema-file ./.schema/question-schema.json --data-file "$output_file"

    # Check if Validate command succeeded
    if [ $? -eq 0 ]; then
        echo "Validation successful"
        # Run Upload command using the same output data file path
        ./target/debug/aarya_cli upload --course-id 1000 --chapter-id 1 --topic-id 1 --data-file "$output_file"
        
        # Check if Upload command succeeded
        if [ $? -eq 0 ]; then
            echo "Upload successful"
        else
            echo "Error: Upload failed"
            exit 1
        fi
    else
        echo "Error: Validation failed"
        exit 1
    fi
else
    echo "Error: Autogen command failed"
    exit 1
fi
