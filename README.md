# Flatten

A command-line utility to flatten all files from a Git repository into a single text file. This tool is particularly useful for feeding entire codebases into Large Language Models (LLMs) for analysis, documentation, or code review purposes.

## Features

- 🔍 **Git-aware**: Works directly with Git repositories and specific revisions
- 📁 **Complete file extraction**: Processes all files from any Git commit, branch, or tag
- 🚫 **Binary file handling**: Automatically detects and skips binary files with placeholders
- 📝 **Clear file separation**: Each file is clearly marked with headers for easy navigation
- ⚡ **Fast and efficient**: Uses Git's native object database for optimal performance
- 🛠️ **Flexible output**: Customizable output file location

## Installation

### From Source

```bash
git clone https://github.com/irvingoujAtDevolution/flatten.git
cd flatten
cargo build --release
```

The binary will be available at `target/release/flatten` (or `target/release/flatten.exe` on Windows).

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Flatten the current repository to default output file
flatten

# Flatten a specific repository
flatten --repo /path/to/repository

# Flatten a specific branch or commit
flatten --rev main
flatten --rev abc123def

# Specify custom output file
flatten --output my_codebase.txt
```

### Command Line Options

```
Options:
  -r, --repo <REPO_PATH>        The path to the git repository [default: .]
  -v, --rev <REVISION>          The git revision to inspect (branch, tag, or commit hash) [default: HEAD]
  -o, --output <OUTPUT_FILE>    The path for the output file [default: flattened_files.txt]
  -h, --help                    Print help
  -V, --version                 Print version
```

### Examples

```bash
# Flatten the main branch of a repository
flatten --repo ./my-project --rev main --output codebase.txt

# Flatten a specific commit
flatten --rev a1b2c3d4 --output snapshot.txt

# Flatten current repository with custom output
flatten -o project_dump.txt
```

## Output Format

The tool creates a text file with the following structure:

```
--- File: src/main.rs ---
[file content here]

--- File: README.md ---
[file content here]

--- File: assets/image.png ---
[Binary file: content not included]

```

## Use Cases

- **LLM Analysis**: Feed entire codebases to language models for code review, documentation generation, or refactoring suggestions
- **Code Documentation**: Create comprehensive snapshots of project states
- **Legacy Code Analysis**: Extract and analyze historical versions of codebases
- **Code Migration**: Prepare code for migration tools that require flat file inputs
- **Educational Purposes**: Create teaching materials from real-world codebases

## Technical Details

- **Language**: Rust 2024 Edition
- **Dependencies**: 
  - `clap` - Command-line argument parsing
  - `git2` - Git repository access
  - `anyhow` - Error handling
- **Binary Detection**: Automatically identifies binary files and excludes their content
- **Memory Efficient**: Streams file content directly to output without loading entire repository into memory

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](http://opensource.org/licenses/MIT))

at your option.

## Author

Irving Ou - [jou@devolutions.net](mailto:jou@devolutions.net)

---

**Note**: This tool respects Git's ignore rules by default and will only process files that are tracked in the Git repository at the specified revision.