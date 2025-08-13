# Kemet - File Content Search Utility

A fast and efficient command-line tool written in Rust for searching text content across multiple files and directories.

## Features

- 🔍 **Recursive Search**: Search through directories and all subdirectories
- 📄 **Multiple File Types**: Support for common file extensions (configurable)
- 🎯 **Case Sensitivity**: Optional case-sensitive search
- 📝 **Line Content Display**: Option to show matching line content
- 💾 **Export Results**: Save search results to a text file
- 🚀 **Fast Performance**: Built with Rust for optimal speed
- 🛡️ **Error Handling**: Robust error reporting and handling

## Installation

### From Source

1. Make sure you have [Rust](https://rustup.rs/) installed
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/kemet.git
   cd kemet
   ```
3. Build and install:
   ```bash
   cargo build --release
   # The binary will be available at target/release/kemet
   ```

### Using Cargo

```bash
cargo install --path .
```

## Usage

```
kemet - File Content Search Utility

USAGE:
    kemet -s <search_text> [OPTIONS]

OPTIONS:
    -p, --path <PATH>           Directory to search (default: current directory)
    -s, --search <TEXT>         Text to search for (required)
    -e, --extensions <EXT>      Comma-separated file extensions (default: txt,json,cs,sql,config,rs,py,js,ts,html,css,xml)
    -o, --output <FILE>         Output file path (if not provided, results shown on console)
    -c, --case-sensitive        Enable case-sensitive search
    -l, --show-lines           Show matching line content
    -h, --help                 Show this help message
```

## Examples

### Basic Search
Search for "function" in the current directory:
```bash
kemet -s "function"
```

### Search Specific Directory
Search for "TODO" in a specific directory:
```bash
kemet -p /home/user/code -s "TODO"
```

### Custom File Extensions
Search only in Rust, Python, and JavaScript files:
```bash
kemet -s "error" -e "rs,py,js"
```

### Case-Sensitive Search with Line Content
```bash
kemet -s "Error" -c -l
```

### Export Results to File
Save search results to a text file:
```bash
kemet -s "function" -o search_results.txt
```

### Complex Search
Search for "async" in a specific directory, show line content, case-sensitive, and export to file:
```bash
kemet -p ./src -s "async" -c -l -o async_functions.txt
```

## Default File Extensions

Kemet searches the following file types by default:
- `.txt` - Text files
- `.json` - JSON files
- `.cs` - C# files
- `.sql` - SQL files
- `.config` - Configuration files
- `.rs` - Rust files
- `.py` - Python files
- `.js` - JavaScript files
- `.ts` - TypeScript files
- `.html` - HTML files
- `.css` - CSS files
- `.xml` - XML files

You can override these with the `-e` option.

## Output Format

### Console Output
```
Searching for "function" in /home/user/project and all subfolders...
Extensions: .rs, .py, .js

Found 15 matches in 8 files:

src/main.rs (Line 42): pub fn main() {
src/utils.rs (Line 15): fn helper_function() -> String {
...

Summary: 23 files searched, 15 matches found
```

### File Output
When using the `-o` option, results are saved to the specified file with the same format, and a confirmation message is shown on the console.

## Performance

Kemet is built with performance in mind:
- Efficient file traversal using Rust's standard library
- Minimal memory footprint
- Fast text searching algorithms
- Concurrent directory traversal (when safe)

## Error Handling

Kemet gracefully handles various error conditions:
- Permission denied errors
- Corrupted or unreadable files
- Invalid file paths
- I/O errors

All errors are reported in the summary without stopping the search process.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v1.0.0
- Initial release
- Basic text search functionality
- Recursive directory traversal
- Multiple file extension support
- Case-sensitive search option
- Line content display option
- Export to file functionality

## Author

Your Name - [@yourusername](https://github.com/yourusername)

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Inspired by grep and other command-line search tools
