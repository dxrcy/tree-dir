use std::fs;

// Formatting lines (with spaces for padding)
const VERT: &str = "│  "; // Vertical line (no branch here)
const FORK: &str = "├─ "; // Forked line (branch here)
const BEND: &str = "└─ "; // Bent line (last branch here)

/// Create tree of directory
pub fn tree(path: &str) -> String {
    path.replace("\\", "/") // Replace backslashes
   + "/\n"  // Add trailing slash to root directory name
   + &branch(path, vec![]).trim()
}

/// Create single branch of tree
fn branch(path: &str, layers: Vec<&str>) -> String {
    let mut output = String::from("");

    // Get vector of directory
    let dir = fs::read_dir(path)
        .expect("Could not read directory")
        .collect::<Vec<Result<fs::DirEntry, std::io::Error>>>();

    // Loop over files and directories in path
    for (i, file) in dir.iter().enumerate() {
        // If is valid file
        if let Ok(file) = file {
            // Check if file is directory
            let is_dir = file
                .metadata()
                .expect("Could not read file or directory metadata")
                .is_dir();

            // Format lines from layers
            let lines = layers.join("") + if i + 1 < dir.len() { FORK } else { BEND };

            // Print name, with trailing slash if is directory
            output.push_str(&format!(
                "{lines}{name}{slash}\n",
                name = file.file_name().to_str().expect("Could not read file name"),
                slash = if is_dir { "/" } else { "" },
            ));

            // Check if file is a directory
            if is_dir {
                // Add new branch
                output.push_str(&branch(
                    file.path().to_str().expect("Could not read directory path"),
                    // Create temporary vector for extra layer
                    {
                        let mut layers = layers.clone();
                        layers.push(if i + 1 < dir.len() { VERT } else { "   " });
                        layers
                    },
                ));
            }
        }
    }

    output
}
