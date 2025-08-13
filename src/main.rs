use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Configuration for the search operation
#[derive(Debug)]
struct Config {
    path: PathBuf,
    search_text: String,
    extensions: Vec<String>,
    case_sensitive: bool,
    show_line_content: bool,
    output_file: Option<PathBuf>,
}

impl Config {
    fn new() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            return Err(Self::usage());
        }

        let mut path: Option<String> = None;
        let mut search_text: Option<String> = None;
        let mut extensions: Option<Vec<String>> = None;
        let mut case_sensitive = false;
        let mut show_line_content = false;
        let mut output_file: Option<PathBuf> = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-p" | "--path" => {
                    path = Self::get_next_arg(&args, &mut i, "path")?;
                }
                "-s" | "--search" => {
                    search_text = Self::get_next_arg(&args, &mut i, "search text")?;
                }
                "-e" | "--extensions" => {
                    if let Some(ext_arg) = Self::get_next_arg(&args, &mut i, "extensions")? {
                        extensions = Some(
                            ext_arg
                                .split(',')
                                .map(|s| {
                                    let trimmed = s.trim();
                                    if trimmed.starts_with('.') {
                                        trimmed.to_string()
                                    } else {
                                        format!(".{}", trimmed)
                                    }
                                })
                                .collect(),
                        );
                    }
                }
                "-o" | "--output" => {
                    if let Some(output_path) = Self::get_next_arg(&args, &mut i, "output file")? {
                        output_file = Some(PathBuf::from(output_path));
                    }
                }
                "-c" | "--case-sensitive" => {
                    case_sensitive = true;
                    i += 1;
                }
                "-l" | "--show-lines" => {
                    show_line_content = true;
                    i += 1;
                }
                "-h" | "--help" => {
                    return Err(Self::usage());
                }
                _ => {
                    return Err(format!(
                        "Unknown argument: {}\n\n{}",
                        args[i],
                        Self::usage()
                    ));
                }
            }
        }

        let path = Self::resolve_path(path)?;
        let search_text =
            search_text.ok_or_else(|| format!("Search text is required\n\n{}", Self::usage()))?;

        let extensions = extensions.unwrap_or_else(|| {
            vec![
                ".txt".to_string(),
                ".json".to_string(),
                ".cs".to_string(),
                ".sql".to_string(),
                ".config".to_string(),
                ".rs".to_string(),
                ".py".to_string(),
                ".js".to_string(),
                ".ts".to_string(),
                ".html".to_string(),
                ".css".to_string(),
                ".xml".to_string(),
            ]
        });

        Ok(Config {
            path,
            search_text,
            extensions,
            case_sensitive,
            show_line_content,
            output_file,
        })
    }

    fn get_next_arg(
        args: &[String],
        i: &mut usize,
        arg_name: &str,
    ) -> Result<Option<String>, String> {
        if *i + 1 < args.len() {
            let value = args[*i + 1].clone();
            *i += 2;
            if value.trim().is_empty() {
                return Err(format!("Empty {} provided", arg_name));
            }
            Ok(Some(value))
        } else {
            Err(format!("Missing value for {}", arg_name))
        }
    }

    fn resolve_path(path: Option<String>) -> Result<PathBuf, String> {
        let path_str = match path {
            Some(p) if !p.trim().is_empty() && p != "." && p != "*" => p,
            _ => env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .to_string_lossy()
                .to_string(),
        };

        let path_buf = PathBuf::from(&path_str);

        if !path_buf.exists() {
            return Err(format!("Path does not exist: {}", path_str));
        }

        if !path_buf.is_dir() {
            return Err(format!("Path is not a directory: {}", path_str));
        }

        Ok(path_buf)
    }

    fn usage() -> String {
        r#"kemet - File Content Search Utility

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

EXAMPLES:
    kemet -s "function"
    kemet -p /home/user/code -s "TODO" -e "rs,py,js"
    kemet -s "Error" -c -l
    kemet -s "function" -o results.txt"#.to_string()
    }
}

/// Output writer that can write to either console or file
enum OutputWriter {
    Console,
    File(File),
}

impl OutputWriter {
    fn new(output_file: Option<&PathBuf>) -> io::Result<Self> {
        match output_file {
            Some(path) => {
                let file = File::create(path)?;
                Ok(OutputWriter::File(file))
            }
            None => Ok(OutputWriter::Console),
        }
    }

    fn writeln(&mut self, text: &str) -> io::Result<()> {
        match self {
            OutputWriter::Console => {
                println!("{}", text);
                Ok(())
            }
            OutputWriter::File(file) => {
                writeln!(file, "{}", text)
            }
        }
    }

    fn write_empty_line(&mut self) -> io::Result<()> {
        self.writeln("")
    }
}

/// Result of a search match
#[derive(Debug)]
struct Match {
    file_path: PathBuf,
    line_number: usize,
    line_content: Option<String>,
}

impl Match {
    fn new(file_path: PathBuf, line_number: usize, line_content: Option<String>) -> Self {
        Self {
            file_path,
            line_number,
            line_content,
        }
    }

    fn format_output(&self, config: &Config) -> String {
        if config.show_line_content {
            if let Some(ref content) = self.line_content {
                format!(
                    "{} (Line {}): {}",
                    self.file_path.display(),
                    self.line_number,
                    content.trim()
                )
            } else {
                format!("{} (Line {})", self.file_path.display(), self.line_number)
            }
        } else {
            format!("{} (Line {})", self.file_path.display(), self.line_number)
        }
    }
}

/// Main search engine
struct SearchEngine<'a> {
    config: &'a Config,
    matches: Vec<Match>,
    files_searched: usize,
    errors: Vec<String>,
}

impl<'a> SearchEngine<'a> {
    fn new(config: &'a Config) -> Self {
        Self {
            config,
            matches: Vec::new(),
            files_searched: 0,
            errors: Vec::new(),
        }
    }

    fn search(&mut self) -> io::Result<()> {
        // Create output writer
        let mut writer = OutputWriter::new(self.config.output_file.as_ref())?;

        writer.writeln(&format!(
            "Searching for \"{}\" in {} and all subfolders...",
            self.config.search_text,
            self.config.path.display()
        ))?;

        if self.config.case_sensitive {
            writer.writeln("Case-sensitive search enabled")?;
        }

        writer.writeln(&format!(
            "Extensions: {}",
            self.config.extensions.join(", ")
        ))?;
        writer.write_empty_line()?;

        self.visit_dir(&self.config.path)?;

        // Display results
        if self.matches.is_empty() {
            writer.writeln("No matches found.")?;
        } else {
            writer.writeln(&format!(
                "Found {} matches in {} files:",
                self.matches.len(),
                self.files_searched
            ))?;
            writer.write_empty_line()?;

            for match_result in &self.matches {
                writer.writeln(&match_result.format_output(self.config))?;
            }
        }

        // Display summary
        writer.write_empty_line()?;
        writer.writeln(&format!(
            "Summary: {} files searched, {} matches found",
            self.files_searched,
            self.matches.len()
        ))?;

        if !self.errors.is_empty() {
            writer.write_empty_line()?;
            writer.writeln("Errors encountered:")?;
            for error in &self.errors {
                writer.writeln(&format!("  {}", error))?;
            }
        }

        // If output was written to file, inform the user
        if let Some(output_path) = &self.config.output_file {
            println!("Results have been written to: {}", output_path.display());
        }

        Ok(())
    }

    fn visit_dir(&mut self, dir: &Path) -> io::Result<()> {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) => {
                self.errors
                    .push(format!("Could not read directory {}: {}", dir.display(), e));
                return Ok(());
            }
        };

        for entry_result in entries {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(e) => {
                    self.errors
                        .push(format!("Could not read entry in {}: {}", dir.display(), e));
                    continue;
                }
            };

            let path = entry.path();

            if path.is_dir() {
                // Recursively search subdirectories
                if let Err(e) = self.visit_dir(&path) {
                    self.errors.push(format!(
                        "Error searching directory {}: {}",
                        path.display(),
                        e
                    ));
                }
            } else if path.is_file() && self.should_search_file(&path) {
                self.search_in_file(&path);
            }
        }

        Ok(())
    }

    fn should_search_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(OsStr::to_str)
            .map(|ext| {
                let ext_with_dot = format!(".{}", ext);
                self.config
                    .extensions
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(&ext_with_dot))
            })
            .unwrap_or(false)
    }

    fn search_in_file(&mut self, file_path: &Path) {
        let file = match fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                self.errors.push(format!(
                    "Could not open file {}: {}",
                    file_path.display(),
                    e
                ));
                return;
            }
        };

        self.files_searched += 1;
        let reader = BufReader::new(file);

        let search_text = if self.config.case_sensitive {
            self.config.search_text.clone()
        } else {
            self.config.search_text.to_lowercase()
        };

        for (line_number, line_result) in reader.lines().enumerate() {
            match line_result {
                Ok(line) => {
                    let line_to_check = if self.config.case_sensitive {
                        line.clone()
                    } else {
                        line.to_lowercase()
                    };

                    if line_to_check.contains(&search_text) {
                        let line_content = if self.config.show_line_content {
                            Some(line)
                        } else {
                            None
                        };

                        self.matches.push(Match::new(
                            file_path.to_path_buf(),
                            line_number + 1,
                            line_content,
                        ));
                    }
                }
                Err(e) => {
                    self.errors.push(format!(
                        "Could not read line {} in file {}: {}",
                        line_number + 1,
                        file_path.display(),
                        e
                    ));
                    break;
                }
            }
        }
    }
}

fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let mut search_engine = SearchEngine::new(&config);

    if let Err(e) = search_engine.search() {
        eprintln!("Search failed: {}", e);
        std::process::exit(1);
    }
}
