use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const DEFAULT_WARN_FILE_LINES: usize = 600;
const README_DEPENDENCY_NAME: &str = "template";

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[ERROR] {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(command) = args.first().map(String::as_str) else {
        print_help();
        return Ok(());
    };

    match command {
        "size" => run_size_check(SizeConfig::from_args(&args[1..])?),
        "update-readme-version" => update_readme_version(&args[1..]),
        "help" | "-h" | "--help" => {
            print_help();
            Ok(())
        }
        other => Err(format!("unknown xtask command: {other}").into()),
    }
}

fn print_help() {
    println!("xtask commands:");
    println!("  size [--root <dir>] [--glob <glob>] [--warn-file-lines <n>]");
    println!("  update-readme-version <version>");
}

#[derive(Debug, Clone)]
struct SizeConfig {
    root: PathBuf,
    globs: Vec<String>,
    warn_file_lines: usize,
}

impl Default for SizeConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            globs: vec!["crates/**/*.rs".to_owned()],
            warn_file_lines: DEFAULT_WARN_FILE_LINES,
        }
    }
}

impl SizeConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn Error>> {
        let mut config = Self::default();
        let mut index = 0;

        while index < args.len() {
            match args[index].as_str() {
                "--root" => {
                    config.root = PathBuf::from(required_value(args, index)?);
                    index += 2;
                }
                "--glob" => {
                    if config.globs == Self::default().globs {
                        config.globs.clear();
                    }
                    config.globs.push(required_value(args, index)?.to_owned());
                    index += 2;
                }
                "--warn-file-lines" => {
                    config.warn_file_lines = required_value(args, index)?.parse()?;
                    index += 2;
                }
                other => return Err(format!("unknown size option: {other}").into()),
            }
        }

        Ok(config)
    }
}

fn required_value(args: &[String], index: usize) -> Result<&str, Box<dyn Error>> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| format!("missing value for {}", args[index]).into())
}

#[derive(Debug)]
struct FileFinding {
    relpath: String,
    lines: usize,
}

fn run_size_check(config: SizeConfig) -> Result<(), Box<dyn Error>> {
    let root = config.root.canonicalize()?;
    let files = collect_rust_files(&root, &config.globs)?;

    let mut file_warnings = Vec::new();

    for path in &files {
        let relpath = path
            .strip_prefix(&root)?
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(path)?;
        let line_count = source.lines().count();

        if line_count > config.warn_file_lines {
            file_warnings.push(FileFinding {
                relpath,
                lines: line_count,
            });
        }
    }

    println!("[INFO] root={}", root.display());
    println!("[INFO] scanned_files={}", files.len());
    println!("[INFO] file_warn>{}", config.warn_file_lines);

    print_file_findings("[WARN] oversized files", &file_warnings);

    let warning_count = file_warnings.len();
    println!();
    println!("[SUMMARY] warnings={warning_count}");

    Ok(())
}

fn collect_rust_files(root: &Path, globs: &[String]) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    collect_files(root, root, globs, &mut files)?;
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_files(
    root: &Path,
    current: &Path,
    globs: &[String],
    files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if name == ".git" || name == "target" || name == ".codegraph" {
            continue;
        }

        if path.is_dir() {
            collect_files(root, &path, globs, files)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }

        let relpath = path
            .strip_prefix(root)?
            .to_string_lossy()
            .replace('\\', "/");
        if globs.iter().any(|glob| matches_glob(glob, &relpath)) {
            files.push(path);
        }
    }

    Ok(())
}

fn matches_glob(pattern: &str, relpath: &str) -> bool {
    match pattern {
        "crates/**/*.rs" => relpath.starts_with("crates/") && relpath.ends_with(".rs"),
        "**/*.rs" => relpath.ends_with(".rs"),
        exact => relpath == exact,
    }
}

fn print_file_findings(title: &str, findings: &[FileFinding]) {
    if findings.is_empty() {
        return;
    }

    println!();
    println!("{title}");
    for finding in findings {
        println!("{}  {}", finding.lines, finding.relpath);
    }
}

fn update_readme_version(args: &[String]) -> Result<(), Box<dyn Error>> {
    let Some(version) = args.first() else {
        return Err("usage: cargo run -p xtask -- update-readme-version <version>".into());
    };

    let readme = Path::new("README.md");
    let source = fs::read_to_string(readme)?;
    let updated = source
        .lines()
        .map(|line| update_readme_version_line(line, version))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(readme, format!("{updated}\n"))?;
    println!("[INFO] updated README.md version snippets to {version}");
    Ok(())
}

fn update_readme_version_line(line: &str, version: &str) -> String {
    let plain_prefix = format!("{README_DEPENDENCY_NAME} = \"");
    if line.trim_start().starts_with(&plain_prefix) {
        return format!("{README_DEPENDENCY_NAME} = \"{version}\"");
    }

    let inline_prefix = [README_DEPENDENCY_NAME, " = ", "{", " version = \""].concat();
    if line.contains(&inline_prefix) {
        return format!(
            "{README_DEPENDENCY_NAME} = {open} version = \"{version}\" {close}",
            open = "{",
            close = "}"
        );
    }

    line.to_owned()
}
