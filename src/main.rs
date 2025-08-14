//! A utility to flatten all files from a repo into a single text file.
//! This tool is useful for creating a single-file to feed into language models.


use anyhow::{Context, Result};
use clap::Parser;
use git2::{ObjectType, Repository, TreeWalkResult};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the git repository.
    #[arg(short, long, value_name = "REPO_PATH", default_value = ".")]
    repo: PathBuf,

    /// The git revision to inspect (e.g., a branch, tag, or commit hash).
    #[arg(long, value_name = "REVISION", default_value = "HEAD")]
    rev: String,

    /// The path for the output file.
    #[arg(
        short,
        long,
        value_name = "OUTPUT_FILE",
        default_value = "flattened_files.txt"
    )]
    output: PathBuf,

    /// Only flatten files under this path (relative to repository root).
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    eprintln!(
        "🔍 Opening repository at: {}",
        cli.repo.canonicalize()?.display()
    );

    // Open the repository at the specified path.
    let repo = Repository::open(&cli.repo)
        .with_context(|| format!("Failed to open repository at '{}'", cli.repo.display()))?;

    // Get the object that the user-specified revision points to.
    let rev_obj = repo
        .revparse_single(&cli.rev)
        .with_context(|| format!("Failed to find revision '{}'", cli.rev))?;

    // Peel the object to a commit.
    let commit = rev_obj
        .peel_to_commit()
        .with_context(|| format!("'{}' could not be peeled to a commit", cli.rev))?;

    eprintln!(
        "🎯 Targeting commit {} ({})",
        commit.id(),
        commit.summary().unwrap_or("No commit summary")
    );

    // Get the tree of the commit.
    let tree = commit.tree()?;

    // Set up the buffered writer for the output file.
    let output_file = std::fs::File::create(&cli.output)
        .with_context(|| format!("Failed to create output file '{}'", cli.output.display()))?;
    let mut writer = BufWriter::new(output_file);

    eprintln!("🚶 Walking the repository tree and writing to file...");

    // Walk the tree recursively.
    tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
        // We are only interested in files (blobs).
        if entry.kind() != Some(ObjectType::Blob) {
            return TreeWalkResult::Ok;
        }

        // Combine the root path with the entry name to get the full path.
        let path = Path::new(root).join(entry.name().unwrap_or("<INVALID_UTF8>"));

        // If a path filter is specified, only include files under that path.
        if let Some(ref filter_path) = cli.path {
            if !path.starts_with(filter_path) {
                return TreeWalkResult::Ok;
            }
        }

        eprintln!("  -> Processing: {}", path.display());

        // Get the blob object from the repository using its ID.
        if let Ok(blob) = repo.find_blob(entry.id()) {
            // Write the file header.
            if let Err(_) = writeln!(writer, "--- File: {} ---", path.display()) {
                return TreeWalkResult::Abort;
            }

            // Check if the blob content is binary. If so, write a placeholder.
            // Otherwise, write the actual content.
            if blob.is_binary() {
                if let Err(_) = writeln!(writer, "[Binary file: content not included]\n") {
                    eprintln!("Failed to write binary file placeholder");
                    return TreeWalkResult::Abort;
                }
            } else {
                if let Err(e) = writer.write_all(blob.content()) {
                    eprintln!("Failed to write file content: {}", e);
                    return TreeWalkResult::Abort;
                }
                if let Err(e) = writeln!(writer, "\n") {
                    // Ensure a newline after the content
                    eprintln!("Failed to write newline after file content: {}", e);
                    return TreeWalkResult::Abort;
                }
            }
        }
        TreeWalkResult::Ok
    })?;

    // Ensure all data is written to the file.
    writer.flush()?;

    println!(
        "\n✅ Success! Repository content flattened to: {}",
        cli.output.display()
    );

    Ok(())
}
