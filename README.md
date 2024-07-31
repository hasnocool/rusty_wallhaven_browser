# Wallhaven Browser

Wallhaven Browser is a simple application that allows users to browse and view wallpapers from [Wallhaven](https://wallhaven.cc/) using the `egui` framework. The application provides functionalities to view the latest, top, and random wallpapers, as well as search for wallpapers based on user queries.

## Features

- Browse wallpapers from Wallhaven:
  - Home
  - Latest wallpapers
  - Top wallpapers
  - Random wallpapers
- Search for wallpapers by query
- Click on wallpaper thumbnails to open the corresponding wallpaper URL in the default web browser

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Build

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/wallhaven_browser.git
    cd wallhaven_browser
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

3. Run the application:

    ```sh
    cargo run --release
    ```

## Usage

When you run the application, you will see a window with the heading "Wallhaven Browser" and a toolbar with the following buttons:

- **Home**: Loads the home page wallpapers from Wallhaven.
- **Latest**: Loads the latest wallpapers from Wallhaven.
- **Top**: Loads the top wallpapers from Wallhaven.
- **Random**: Loads random wallpapers from Wallhaven.
- **Search**: Enter a search query in the text box and click this button to search for wallpapers.

The wallpapers will be displayed as thumbnails in a grid layout. You can click on any thumbnail to open the corresponding wallpaper URL in your default web browser.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Wallhaven](https://wallhaven.cc/) for providing the wallpapers.
- [eframe](https://docs.rs/eframe/) and [egui](https://docs.rs/egui/) for the Rust GUI framework.
- [reqwest](https://docs.rs/reqwest/) for the HTTP client.
- [select](https://docs.rs/select/) for HTML parsing.
