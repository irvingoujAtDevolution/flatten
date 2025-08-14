use git2::{Repository, TreeWalkResult, ObjectType};
use std::{io::{BufWriter, Write}, path::{Path, PathBuf}};

fn main() -> Result<(), git2::Error> {
    // Open the repository in the current directory.
    let repo = Repository::open(".")?;

    let output_file = PathBuf::from("flattened_files.txt");
    let output_file = std::fs::File::options().create(true).write(true).open(output_file).expect("Failed to open output file");
    let mut output_file = BufWriter::new(output_file);

    // Get the object that HEAD points to.
    let head_obj = repo.revparse_single("HEAD")?;

    // Peel the object to a commit.
    let commit = head_obj.peel_to_commit()?;

    // Get the tree of the commit.
    let tree = commit.tree()?;

    println!("Files in the current HEAD commit:");

    // Walk the tree and print the path of each file.
    tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
        // We are only interested in files (blobs).
        if entry.kind() == Some(ObjectType::Blob) {
            let path = Path::new(root).join(entry.name().unwrap_or(""));
            if let Ok(content) = std::fs::read(path.clone()) {
                println!("reading file: {}", path.display());
                let Ok(content) = String::from_utf8(content) else {
                    return TreeWalkResult::Ok
                };

                output_file.write(format!("{}\n", path.display()).as_bytes())
                    .expect("Failed to write to output file");

                output_file.write_all(content.as_bytes())
                    .expect("Failed to write file content to output file");

                output_file.write_all(b"=========================\n")
                    .expect("Failed to write separator to output file");


            }
        }
        TreeWalkResult::Ok
    })?;


    Ok(())
}