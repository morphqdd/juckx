
# juckx 🚀

**juckx** is a Rust-based CLI tool that automates the process of creating meaningful git commit messages by leveraging Gemini AI. With one simple command, it stages your changes, generates a concise commit message based on your code diffs or new files, commits, and pushes to the remote repository.

## Features ✨

- ✅ Detects if the current folder is a git repository and handles the initial commit scenario gracefully.  
- 📝 Extracts staged diffs or untracked files to generate an accurate prompt for AI.  
- 🤖 Generates professional and concise commit messages in imperative mood using Gemini AI.  
- 🔍 Supports dry-run mode to preview the generated commit message without committing.  
- 🚫 Allows skipping push with a command-line option.  
- 🌐 Supports specifying commit message language via CLI.  
- 🔑 Supports setting and saving Gemini API key to a `.env` file from the CLI.  
- ⚙️ Automates the full git workflow: add, commit, and push with one command.  
- 🦀 Written in Rust with asynchronous processing powered by Tokio.

## Installation 🛠️

Build from source using Cargo:

```bash
git clone https://github.com/morphqdd/juckx.git
cd juckx
cargo build --release
```

Pre-built binaries coming soon.

## Usage 💻

```bash
juckx [OPTIONS]
```

### Options

- `--dry-run` — Generate and display the commit message without committing or pushing.  
- `--no-push` — Commit changes but skip pushing to remote.  
- `--lang <LANG>` — Specify the language for commit message generation (default: `en`).  
- `--with-api <API_KEY>` — Save Gemini API key to `.env` file for later use.

## Example 🚀

```bash
juckx --with-api YOUR_API_KEY --lang ru
```

This will:  
1. Save the API key to `.env`.  
2. Check if the current directory is a git repo.  
3. Use untracked files if no commits exist, otherwise get the staged diff.  
4. Generate a commit message via Gemini AI.  
5. Show the generated commit message.  
6. Stage all changes.  
7. Commit with the generated message.  
8. Push to the remote repository (unless `--no-push` is specified).

## Contributing 🤝

Contributions and suggestions are welcome! Feel free to open issues or pull requests.

## License 📄

[MIT License © 2025](LICENSE)