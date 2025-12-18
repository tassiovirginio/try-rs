# try-rs 🦀
> **A blazing fast, Rust-based workspace manager for your temporary experiments.**

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-d45500.svg)


**try-rs** is a CLI tool designed to manage the chaos of temporary projects. Instead of cluttering your Desktop or `/tmp` with `test1`, `new-test`, and `final-test`, `try-rs` organizes them into date-prefixed directories, offering a robust TUI (Terminal User Interface) to create, navigate, and clean up your experiments.

## 💡 Inspiration

This project is a Rust port and re-imagination of the excellent [try](https://github.com/tobi/try) tool by **Tobi Lütke**.

While the original is a lightweight Ruby script, **try-rs** aims to bring the same philosophy—"Your experiments deserve a home"—but with the performance, type safety, and modern TUI capabilities (using [Ratatui](https://github.com/ratatui/ratatui)) of the Rust ecosystem.

## 🤔 Why try-rs?

| Feature         | Description                                                                                             |
|-----------------|---------------------------------------------------------------------------------------------------------|
| **🏎️ Blazing Fast** | Built in Rust, it's compiled to a native binary. No more waiting for a script interpreter to start up. |
| **🖼️ Rich TUI**     | A modern Ratatui interface makes browsing and managing projects a breeze.                               |
| **✅ Type Safe**    | Say goodbye to runtime errors. Rust's strict compiler catches them before they become a problem.        |
| **Git-Aware**   | Clones repos with a single command, automatically naming them.                                          |

## ✨ Features

* **Fuzzy Search:** Instantly find old experiments using a smart fuzzy matching algorithm.
* **Auto-Dating:** Creates directories like `2025-12-17-rust-test` automatically.
* **Git Integration:** Paste a GitHub/GitLab URL, and it will auto-clone it into a dated folder.
    * `try-rs https://github.com/ratatui/ratatui` → clones to `.../2025-12-17-ratatui`
* **Terminal UI:** A beautiful TUI built with Ratatui to browse your projects.
* **Safe Deletion:** Delete old experiments directly from the UI (`Ctrl+D`) with a confirmation popup.
* **Configurable:** Supports `XDG Base Directory` specification (`~/.config/try-rs/config.toml`).

## 📦 Installation

### Prerequisites

* Rust & Cargo installed.
* A shell (Fish, Zsh, Bash).

### Building from source

```bash
git clone [https://github.com/tassiovirginio/try-rs.git](https://github.com/tassiovirginio/try-rs)
cd try-rs
cargo install --path . --bin try-rs
```

⚙️ Configuration

1. Setup the Shell Integration

Since try-rs needs to change your shell's current directory, it requires a small wrapper function.

🐟 Fish Shell (Recommended)

Create the file ~/.config/fish/functions/try-rs.fish:

```
function try-rs
    # Captures the output of the binary (stdout) which is the "cd" command
    # The TUI is rendered on stderr, so it doesn't interfere.
    set command (command try-rs $argv | string collect)

    if test -n "$command"
        eval $command
    end
end
```

(Optional) Create an abbreviation:

```
abbr --add try try-rs
```

🐚 Bash / Zsh

Add this to your .bashrc or .zshrc:

```bash
try-rs() {
    # Run the binary and capture stdout
    local cmd=$(command try-rs "$@")
    # Evaluate the cd command
    eval "$cmd"
}
```

2. Config File (Optional)
By default, experiments are stored in ~/src/tries. To change this, create ~/.config/try-rs/config.toml:

```toml
# ~/.config/try-rs/config.toml
tries_path = "~/Development/playground"
```

🚀 Usage

Simply type try-rs (or your alias) in your terminal.

### Key Bindings

| Key      | Action                                               |
|----------|------------------------------------------------------|
| `Type`   | Filter the list (Fuzzy Search)                       |
| `↑` / `↓`| Navigate the list                                    |
| `Enter`  | Select directory (or create new if text doesn't match) |
| `Ctrl+D` | Delete the selected directory (triggers popup)       |
| `Esc`    | Cancel / Close Popup                                 |

### CLI Commands

You can also bypass the UI:

| Command                                      | Description                                  |
|----------------------------------------------|----------------------------------------------|
| `try-rs`                                     | Opens the TUI                                |
| `try-rs <name>`                              | Create (or jump to) a named experiment       |
| `try-rs <https://github.com/user/repo>` | Clones a repository into a dated folder      |

🤝 Contributing
Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

## 🗺️ Roadmap

- [ ] **Workspaces:** Support for multiple, named workspaces (e.g., `try-rs --workspace work`).
- [ ] **Configuration Wizard:** An interactive first-time setup to generate the config file.
- [ ] **More Actions:** Add `rename` and `archive` actions to the TUI.
- [ ] **Theme Support:** Allow users to customize the TUI colors.

📄 License
[MIT](LICENSE)
