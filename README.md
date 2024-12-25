# Micro:bit V2 Game

This is a small, interactive game developed for the **micro:bit v2 microcontroller** using **Rust**. The game utilizes the micro:bit's hardware features, including its display, buttons, and timers, to provide an engaging experience. It also showcases how to efficiently work with embedded systems using the **microbit v2 crate**, **embed crate**, and **critical section** for safe concurrency and real-time operations.

## Features

- **Interactive Gameplay**: The game makes full use of the **micro:bit v2**'s built-in features such as buttons, display, and timers for a responsive gaming experience.
- **Efficient Memory Management**: Leveraging Rust's memory safety features to ensure stable and crash-free gameplay on embedded hardware.
- **Real-Time Operations**: The game utilizes the **embed crate** for managing interrupts and real-time clock (RTC) events using **critical section** to manage concurrency safely.
- **Low Overhead**: The game is designed to be lightweight and efficient, ensuring smooth performance on the microcontroller.

## Technologies Used

- **Rust**: The game is built entirely in Rust, utilizing its powerful embedded ecosystem.
- **microbit v2 crate**: Provides support for the micro:bit v2 hardware, enabling interaction with buttons, LEDs, and other features.
- **embed crate**: Used for embedded programming, including handling real-time operations and safe concurrency.
- **critical section**: Ensures safe access to shared resources in a multi-threaded environment.

## Installation & Setup

To flash the game onto your **micro:bit v2**, follow these steps:

1. **Clone the repository**:

   ```bash
    git clone https://github.com/yourusername/your-game.git
   cd your-game
   ```

2. **Install Rust for Embedded Development**:
Follow the setup instructions for [Rust Embedded](https://rust-embedded.github.io/book/)
3. **Install Dependencies**:
    Ensure you have the necessary dependencies installed. You can do this by running:

    ```bash
    rustup target add thumbv7em-none-eabihf
    cargo install probe-rs
    ```

4. **Flash the Game**:
    Use `probe-rs` to flash the compiled binary onto your **micro:bit v2**:

    ```bash
    cargo embed --release
    ```

## Usage

Once the game is flashed onto your **micro:bit v2**, you can start playing by pressing the buttons on the device.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome!

## Contact

For any questions or feedback, please contact [uwau2544@gmail.com](mailto:uwau2544@gmail.com).

