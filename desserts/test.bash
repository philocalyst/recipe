
#!/bin/bash

# Script to create en_US and assets directories in subdirectories and organize files

# Define common image extensions
image_extensions=("jpg" "jpeg" "png" "gif" "bmp" "tiff" "tif" "webp" "svg" "ico")

# Function to check if a file is an image
is_image() {
    local file="$1"
    local ext="${file##*.}"
    ext=$(echo "$ext" | tr '[:upper:]' '[:lower:]') # Convert to lowercase
    
    for img_ext in "${image_extensions[@]}"; do
        if [[ "$ext" == "$img_ext" ]]; then
            return 0  # True, it's an image
        fi
    done
    return 1  # False, not an image
}

# Loop through each subdirectory (one level down)
for subdir in */; do
    # Remove trailing slash from directory name
    subdir="${subdir%/}"
    
    # Check if it's actually a directory
    if [[ -d "$subdir" ]]; then
        echo "Processing directory: $subdir"
        
        # Create en_US and assets directories inside the subdirectory
        mkdir -p "$subdir/en_US"
        mkdir -p "$subdir/assets"
        
        # Process all files in the subdirectory
        for file in "$subdir"/*; do
            # Skip if it's one of the directories we just created
            if [[ "$file" != "$subdir/en_US" && "$file" != "$subdir/assets" ]]; then
                # Only process files (not directories)
                if [[ -f "$file" ]]; then
                    filename=$(basename "$file")
                    
                    # Check if it's an image file
                    if is_image "$file"; then
                        echo "  Moving image to assets: $filename"
                        mv "$file" "$subdir/assets/"
                    else
                        echo "  Moving file to en_US: $filename"
                        mv "$file" "$subdir/en_US/"
                    fi
                fi
            fi
        done
        
        echo "  Completed: $subdir"
    fi
done

echo "All subdirectories processed!"
