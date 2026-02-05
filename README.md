# Spore Downloader

A command-line tool for downloading Spore creations from either a user or a sporecast.

This is a Rust reimplementation inspired by the
[PNG-Downloader](https://github.com/Spore-Community/PNG-Downloader) from the Spore Community,
providing cross-platform support. This project was created to fill the gap left by the original
tool on Linux and as a learning exercise for building CLI tools in Rust.

### Build from Source

Make sure you have [Rust installed](https://rustup.rs/), then:

```bash
git clone https://github.com/MateusF03/spore-downloader.git
cd spore-downloader
cargo build --release
```

The compiled binary will be available at `target/release/spore-downloader` (or `spore-downloader.exe` on Windows).

## Usage

### Download All Creations from a User

```bash
spore-downloader user <username>
```

This will download all creations from the specified user to `spore_assets/users/<username>/`.

**Example:**
```bash
spore-downloader user JohnDoe
```

**With Custom Output Directory:**
```bash
spore-downloader user JohnDoe --output ./my_backups
```

### Download All Creations from a Sporecast

```bash
spore-downloader sporecast <sporecast-id>
```

This will download all creations from the specified sporecast to `spore_assets/sporecasts/`.

**Example:**
```bash
spore-downloader sporecast 123456789
```

**With Custom Output Directory:**
```bash
spore-downloader sporecast 123456789 --output ./sporecast_backups
```
### Output Structure

By default, downloaded assets are saved as PNG files in directories named after the user or sporecast ID:

```text
spore_assets/
├── users/
│   └── JohnDoe/
│       ├── 123456789.png
│       └── 987654321.png
```
If `--separate-by-type` is enabled, assets are grouped by creation type:
```text
spore_assets/
├── users/
│   └── JohnDoe/
│       ├── creatures/
│       ├── vehicles/
│       ├── buildings/
│       ├── ufos/
│       └── adventures/
```

### Options

- `-o, --output <directory>`: Specify a custom output directory
- `-h, --help`: Display help information
- `--separate-by-type`: Organize downloaded assets into folders by creation type

## Acknowledgments

This project is inspired by the [Spore PNG Downloader](https://github.com/Spore-Community/PNG-Downloader) by the Spore Community.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Not associated with or endorsed by Electronic Arts or Maxis.*
