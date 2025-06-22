use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

fn main() {
    let project_path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    let cargo_toml_path = project_path.join("Cargo.toml");
    let cargo_toml_content = match fs::read_to_string(&cargo_toml_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("❌ Failed to read Cargo.toml: {}", err);
            return;
        }
    };

    let cargo_toml: toml::Value = match toml::from_str(&cargo_toml_content) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("❌ Failed to parse Cargo.toml: {}", err);
            return;
        }
    };

    if let Some(name) = cargo_toml.get("package").and_then(|pkg| pkg.get("name")) {
        println!("📦 Package: {}", name.as_str().unwrap_or("unknown"));
    } else {
        println!("📦 Package: <unknown>");
    }

    let src_path = project_path.join("src");

    // Match mod declarations like: mod foo; or mod foo {
    let mod_regex = Regex::new(r"^\s*mod\s+([a-zA-Z0-9_]+)\s*[{;]").unwrap();

    // lib.rs
    let lib_path = src_path.join("lib.rs");
    if lib_path.exists() {
        println!("└── lib (src/lib.rs) [crate]");
        print_mods(&lib_path, 1, &mod_regex);
    }

    // main.rs
    let main_path = src_path.join("main.rs");
    if main_path.exists() {
        println!("└── bin (src/main.rs) [crate]");
        print_mods(&main_path, 1, &mod_regex);
    }

    // src/bin/*.rs
    let bin_dir_path = src_path.join("bin");
    if bin_dir_path.exists() && bin_dir_path.is_dir() {
        if let Ok(entries) = fs::read_dir(&bin_dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    let filename = path.file_stem().unwrap_or_default().to_string_lossy();
                    println!(
                        "└── bin: {} (src/bin/{}) [crate]",
                        filename,
                        path.file_name().unwrap().to_string_lossy()
                    );
                    print_mods(&path, 1, &mod_regex);
                }
            }
        }
    }
}

fn print_mods(file_path: &Path, base_indent: usize, regex: &Regex) {
    if let Ok(contents) = fs::read_to_string(file_path) {
        let mut indent_stack = vec![base_indent];
        let mut current_indent = base_indent;

        for line in contents.lines() {
            let trimmed = line.trim();

            // Adjust indentation based on braces
            if trimmed.contains('}') {
                indent_stack.pop();
                current_indent = *indent_stack.last().unwrap_or(&base_indent);
            }

            if let Some(captures) = regex.captures(trimmed) {
                let mod_name = captures.get(1).map(|m| m.as_str()).unwrap_or("<unknown>");
                let is_inline = trimmed.ends_with('{');
                let tag = if is_inline { "[mod: inline]" } else { "[mod: ext]" };

                println!("{}└── {} {}", "    ".repeat(current_indent), mod_name, tag);

                if is_inline {
                    indent_stack.push(current_indent + 1);
                    current_indent += 1;
                } else {
                    // External module — recurse into file
                    let parent = file_path.parent().unwrap_or(Path::new("."));
                    let mod_rs = parent.join(format!("{}.rs", mod_name));
                    let mod_dir_rs = parent.join(mod_name).join("mod.rs");

                    if mod_rs.exists() {
                        print_mods(&mod_rs, current_indent + 1, regex);
                    } else if mod_dir_rs.exists() {
                        print_mods(&mod_dir_rs, current_indent + 1, regex);
                    }
                }
            }

            if trimmed.contains('{') && !trimmed.starts_with("mod ") {
                indent_stack.push(current_indent + 1);
                current_indent += 1;
            }
        }
    }
}


