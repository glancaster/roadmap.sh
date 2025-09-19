# gh-activity

Inspired by [roadmap.sh/projects/github-user-activity](https://roadmap.sh/projects/github-user-activity).

A Rust CLI tool to fetch and display recent GitHub user activity.

## Setup

1. Ensure Rust is installed on your system.
2. Clone or download the project.
3. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the tool with a GitHub username as an argument:

```
cargo run <username>
```

Or with the built binary:

```
./target/release/gh-activity <username>
```

## Examples

Fetch activity for user "octocat":

```
cargo run octocat
```

Output example:
```
Github Username: octocat
- Starred 5 Repositories
  - (2) octocat/Hello-World
  - (1) octocat/Spoon-Knife
- Pushed 3 Commits
  - (3) octocat/Hello-World
```

## Requirements

- Rust (latest stable version)
- Internet connection for API calls