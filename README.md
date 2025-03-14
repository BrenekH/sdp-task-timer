# Senior Design Task Timer

Terminal user interface (TUI) for timing work completed on tasks, intended for use with Boise State's senior design project.

## Installation

### Dependencies

* [Rust](https://rustup.rs)

* [Authenticated GitHub CLI](https://cli.github.com)

### Installing

Running `cargo install --git https://github.com/BrenekH/sdp-task-timer.git` will install `sdp-task-timer` into `~/.cargo/bin` (for a default Rust installation).
If you can't run `sdp-task-timer` after installing with Cargo, make sure `~/.cargo/bin` is in your PATH.

To uninstall, run `cargo uninstall sdp-task-timer`.

## Usage

Running `sdp-task-timer` for the first time will present you with a prompt to enter the repository to pull tasks from.
The format is `<organization>/<repo>`, which for most teams will be `cs481-ekh/<s|f><year>-<team name>`.
This is only needed once, and your answer is stored in `{CONFIG_DIR*}/sdp-task-timer/config.toml`.

Every time `sdp-task-timer` is run, it will ask which issue you want to track.
The tasks in this list are open issues that have the `task` label and are either assigned to you, or not assigned to anyone.
If you want to view closed issues as well, pass the `--all` cli argument (ex. `sdp-task-timer --all`).
