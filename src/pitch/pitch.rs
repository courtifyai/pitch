use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Item {
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Target {
    from: String,
    to: String,
    regex: bool,
    targets: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config {
    shift: Vec<Item>,
    patch: Vec<Item>,
    alter: Vec<Target>,
}

pub struct Options {
    pub config: String,
}

pub struct Pitch {
    config: Config,
}

impl Pitch {
    pub fn new(options: Options) -> Self {
        let r = File::open(&options.config).expect("Failed to open file");
        let config: Config = serde_yaml::from_reader(r).unwrap();

        Self { config }
    }

    pub fn copy(&self, from: &str, to: &str, action: &str) {
        let mut to_replaced = to.to_string();

        for item in &self.config.alter {
            for target in &item.targets {
                if to.contains(target) {
                    let replaced = if item.regex {
                        let re = Regex::new(&item.from).unwrap();
                        re.replace_all(&to_replaced, &item.to).to_string()
                    } else {
                        to_replaced.replace(&item.from, &item.to)
                    };

                    if to_replaced != replaced {
                        to_replaced = replaced;
                        println!("🔄 (r) {} → {}", to, to_replaced);
                    }
                }
            }
        }

        if Path::new(&to_replaced).exists() {
            if Path::new(&to_replaced).is_dir() {
                fs::remove_dir_all(&to_replaced).unwrap();
            } else {
                fs::remove_file(&to_replaced).unwrap();
            }
        }

        if let Some(parent) = Path::new(&to_replaced).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        match action {
            "s" => println!("📦 (s) {} → {}", from, to),
            "p" => println!("⚙️ (p) {} → {}", from, to),
            _ => println!("{}", action),
        }

        fs::copy(from, &to_replaced).unwrap();

        // Skip non-text files (invalid UTF-8)
        let Ok(content) = fs::read_to_string(&to_replaced) else {
            return;
        };

        let mut content_replaced = content.to_string();

        for item in &self.config.alter {
            for target in &item.targets {
                if to.contains(target) {
                    let replaced = if item.regex {
                        let re = Regex::new(&item.from).unwrap();
                        re.replace_all(&content_replaced, &item.to).to_string()
                    } else {
                        content_replaced.replace(&item.from, &item.to)
                    };

                    if content_replaced != replaced {
                        content_replaced = replaced;
                        println!("✏️ (r) {} → {}", item.from, item.to);
                    }
                }
            }
        }

        fs::write(&to_replaced, &content_replaced).unwrap();
    }

    pub fn shift(&self) {
        for item in &self.config.shift {
            let to = item.to.clone();
            let path = Path::new(&item.from);

            if path.is_dir() {
                // Recursively collect all files in the directory
                for entry in
                    WalkDir::new(path).into_iter().filter_map(|e| e.ok())
                {
                    if entry.file_type().is_file() {
                        // Get relative path by stripping the source directory prefix
                        let relative = entry.path().strip_prefix(path).unwrap();
                        let to_path = Path::new(&to).join(relative);

                        self.copy(
                            entry.path().to_str().unwrap(),
                            to_path.to_str().unwrap(),
                            "s",
                        );
                    }
                }
            } else if path.is_file() {
                self.copy(path.to_str().unwrap(), &to, "s");
            }
        }
    }

    pub fn patch(&self) {
        for item in &self.config.patch {
            let to = item.to.clone();
            let path = Path::new(&item.from);

            if path.is_dir() {
                for entry in
                    WalkDir::new(path).into_iter().filter_map(|e| e.ok())
                {
                    if entry.file_type().is_file() {
                        // Get relative path by stripping the source directory prefix
                        let relative = entry.path().strip_prefix(path).unwrap();
                        let to_path = Path::new(&to).join(relative);

                        self.copy(
                            entry.path().to_str().unwrap(),
                            to_path.to_str().unwrap(),
                            "p",
                        );
                    }
                }
            } else if path.is_file() {
                self.copy(path.to_str().unwrap(), &to, "p");
            }
        }
    }
}
