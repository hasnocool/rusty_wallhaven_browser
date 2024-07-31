# CHANGELOG

## [Unreleased] - 7.31.2024

### Added
- Initial implementation of `Wallhaven Browser` using `egui` framework.
- Application UI with the following components:
  - Heading: "Wallhaven Browser"
  - Horizontal toolbar with buttons: "Home", "Latest", "Top", "Random", and "Search".
  - Text input for search queries.
  - Vertical scroll area for displaying wallpaper thumbnails.
- Functionality to load wallpapers from the following URLs:
  - Home: `https://wallhaven.cc/`
  - Latest wallpapers: `https://wallhaven.cc/latest`
  - Top wallpapers: `https://wallhaven.cc/toplist`
  - Random wallpapers: `https://wallhaven.cc/random`
  - Search wallpapers: `https://wallhaven.cc/search?q=<query>`
- Functionality to click on wallpaper thumbnails to open the corresponding wallpaper URL in the default web browser.

### Changed
- Refactored the initial loading of wallpapers to be triggered on application startup.
- Changed the method of displaying images from `egui::ImageButton` to `egui::Image` for better compatibility.
- Improved the grid layout logic to ensure uniform spacing and padding between thumbnails.
- Removed unnecessary grid and responsive image logic to simplify the code.

### Fixed
- Fixed issue with incorrect arguments being passed to `egui::ImageButton::new`.
- Fixed padding issues by ensuring uniform spacing between thumbnails.
- Fixed potential division by zero error in the grid layout calculation.

### Deprecated
- Removed the responsive grid logic to simplify the layout and ensure consistent spacing.

### Notes
- The `load_wallpapers` function fetches and parses wallpaper thumbnails from the specified URL using `reqwest` and `select` crates.
- The `load_image_from_url` function loads images from URLs and creates texture handles for rendering in the `egui` context.
- The application now ensures that all UI components and functionalities are correctly integrated and functioning as expected.
