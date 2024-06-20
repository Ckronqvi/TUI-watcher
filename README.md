# TUI Watcher

TUI Watcher is a command-line tool designed to monitor TUI prices and notify users via email when the price changes. It provides functionality to fetch the current price, continuously watch for price updates, and optionally daemonize the watcher process. I made this because I have "hintatakuu plus" and I did not like to manually check the price from the website.

## Features

- **Get Price \<url\>** : Fetches the current price of a specified TUI package URL.
- **Watch \<url\> \<email address\>** : Starts a watcher that monitors the price continuously. It sends an email notification whenever the price changes.
- **Watch \<url\> \<email address\> --daemon** : Optionally daemonizes the watcher process for running in the background.

## Requirements

- Rust programming language (https://www.rust-lang.org/)
- Cargo package manager (comes with Rust installation)
- External dependencies are managed through Cargo.toml.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Ckronqvi/TUI-watcher.git
   cd tui-watcher
   ```
2. Create .env file to input your smpt credentials:

   ```bash
   touch .env

   # Example .env file
   EMAIL=first.last@mail.com         # this is the sender email
   CRED_PASSWORD=password            # this is smtp password
   CRED_EMAIL=first.last@mail.com    # this is smtp credential email
   ```

3. Build:
   ```bash
   cargo build
   ```
4. Run:

   ```bash
   # Returns the current price
   cargo run -- get-price <url>

   # Check every 2 hours if the price of the holiday package has changed. It will notify you via email if any change occurs.
   cargo run -- watch <url> <email>

   # Same as above but it will run as a background process
   cargo run -- watch <url> <email> --daemon

   # Stops the daemon
   cargo run -- stop
   # Alternative way to stop
   kill $(cat /tmp/tui-watcher.pid)
   ```

## Disclaimer

This tool is for educational purposes only. It is provided "as is" without warranty of any kind. The author is not responsible for any misuse or damage caused by this tool. Use it at your own risk.
