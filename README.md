# chkdeps

Command-line tool to locally check your software project's dependencies against the latest versions on the remote package registries.

## Config
This project uses the [Libraries.io](https://libraries.io) dataset, and as such requires an API key from them. You can set it one of these folders and creating a `config.toml` file for the application to use:

| Platform          | Value |
|-------------------|-------|
| GNU/Linux         | `$XDG_DATA_HOME` or `$HOME`/.local/share     |
| Apple macOS       | `$HOME`/Library/Application Support     |
| Microsoft Windows | `{FOLDERID_LocalAppData}` / `%LOCALAPPDATA%`     |

### Example
```toml
[librariesio]
api_key = "APIkeyGoesHere"
```
## Usage
```
$> chkdeps .\Cargo.toml

✔ 🦀 lenient_semver: 0.4.2 - 0.4.1
❌🦀 serde_json: 1.0.57 - 1.0.108
❌🦀 either: 1.7.0 - 1.9.0
❌🦀 semver: 1.0.16 - 1.0.20
❌🦀 dirs: 4.0.0 - 5.0.1
❌🦀 serde: 1.0.145 - 1.0.193
❌🦀 tracing: 0.1.31 - 0.1.40
❌🦀 clap: 4.0.29 - 4.4.8
❌🦀 anyhow: 1.0.66 - 1.0.75
❌🦀 termcolor: 1.2.0 - 1.4.0
❌🦀 toml: 0.7.3 - 0.8.8
❌🦀 ureq: 2.5.0 - 2.9.1
```