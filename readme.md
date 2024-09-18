
# Tablet gestures

## Features

- **Change Wallpaper:** Double-tap the screen to change the wallpaper.
- **Adjust Brightness and Volume:** Press and hold the left or right edges of the screen and move the stylus up or down to adjust brightness or volume.
- **Switch Workspaces:** Swipe the stylus in the specified direction to change between workspaces.

## System Requirements

- Linux-based operating system with Wayland.
- ALSA drivers and services for volume control.
- `brillo` command for screen brightness adjustment.
- Convertible laptop, preferably the Samsung Book2 Pro 360.

## Installation

1. Clone this repository to your local machine:
    ```bash
    git clone https://github.com/senchpimy/your-repository
    ```
2. Run the installation script:
    ```bash
    cd your-repository
    ./install.sh
    ```

## Usage

1. Start the program with the `-d` flag to run it as a daemon:
    ```bash
    ./target/release/tablet_utils -d
    ```
2. The program will begin monitoring the input events for the stylus. Use the stylus to interact with the screen as follows:
    - Double-tap the screen to change the wallpaper.
    - Press and hold the left or right edges of the screen and move the stylus up or down to adjust brightness or volume.
    - Swipe in the specified direction to switch workspaces.

## Customization

- Gestures are predefined and can only be modified by reprogramming the software. Adjust the code as needed to define new gestures or actions.

## License

This project is licensed under the GPLv3 License.
