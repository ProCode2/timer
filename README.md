# Timer

This is a tool that I use to time my daily tasks.


## Features

- Display the current month's calendar with the current day highlighted.
- Play a beep sound using a sine wave for the specified duration and frequency.
- Start a countdown timer with a specified duration and display the remaining time in the terminal.

## Requirements

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Dependencies

- `chrono`: For date and time manipulation.
- `rodio`: For audio playback.
- `home`: For determining the user's home directory.

These dependencies are specified in the `Cargo.toml` file:

```toml
[dependencies]
chrono = "0.4"
rodio = "0.15"
home = "0.5"
```

## Installation

1. Ensure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs).
2. Clone the repository:
    ```sh
    git clone https://github.com/procode2/timer.git
    ```
3. Navigate to the project directory:
    ```sh
    cd timer-app
    ```
4. Build the project:
    ```sh
    cargo build --release
    ```
5. Run the project:
    ```sh
    cargo run --release
    ```

## Usage

### Displaying the Calendar

When you run the application, it will display the current month's calendar with the current day highlighted.

### Starting a Timer

To start a countdown timer, pass the `st` argument followed by the duration in `HH:MM:SS` format:

```sh
cargo run --release -- st 00:01:00
```

This will start a 1-minute countdown timer. Once the timer ends, it will play a beep sound.

### Example

```sh
cargo run --release -- st 00:00:10
```

This will start a countdown timer for 10 seconds.

## Code Overview

## Contributing

Feel free to fork this repository, make improvements, and submit pull requests. All contributions are welcome!

## Contact

If you have any questions, feel free to open an issue or contact me.

