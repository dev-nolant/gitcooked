use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn minify_html(html: &str) -> String {
    let mut result = String::new();
    let mut in_script = false;
    let mut in_style = false;
    let mut chars = html.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '<' if chars.peek().map_or(false, |&n| n == 's') => {
                let next_5: String = chars.clone().take(5).collect();
                if next_5 == "script" {
                    in_script = true;
                    result.push_str("<script");
                    chars.by_ref().take(5).for_each(drop);
                    continue;
                }
            }
            '<' if chars.peek().map_or(false, |&n| n == 's') => {
                let next_5: String = chars.clone().take(5).collect();
                if next_5 == "style" {
                    in_style = true;
                    result.push_str("<style");
                    chars.by_ref().take(5).for_each(drop);
                    continue;
                }
            }
            '<' if chars.peek().map_or(false, |&n| n == '/') => {
                let next_7: String = chars.clone().take(7).collect();
                if next_7 == "/script" && in_script {
                    in_script = false;
                    result.push_str("</script>");
                    chars.by_ref().take(7).for_each(drop);
                    continue;
                } else if next_7 == "/style" && in_style {
                    in_style = false;
                    result.push_str("</style>");
                    chars.by_ref().take(7).for_each(drop);
                    continue;
                }
            }
            _ => {}
        }

        if in_script || in_style {
            result.push(c);
        } else if !c.is_whitespace() {
            result.push(c);
        } else if !result.ends_with(' ')
            && !result.ends_with('>')
            && chars
                .peek()
                .map_or(false, |&n| !n.is_whitespace() && n != '<')
        {
            result.push(' ');
        }
    }

    result
}

fn embed_html_as_rust_code(source_path: &Path, name: &str, out_dir: &Path) {
    let html = fs::read_to_string(source_path).expect("Failed to read template");
    let minified = minify_html(&html);
    let escaped = minified.replace('\\', "\\\\").replace('"', "\\\"");

    let rust_code = format!(
        r#"pub const {}: &str = "{}";"#,
        name.to_uppercase(),
        escaped
    );

    let templates_path = out_dir.join("templates.rs");
    let mut content = if templates_path.exists() {
        fs::read_to_string(&templates_path).expect("Failed to read templates.rs")
    } else {
        String::new()
    };
    content.push('\n');
    content.push_str(&rust_code);
    fs::write(&templates_path, content).expect("Failed to write templates.rs");
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let templates_dir = PathBuf::from("templates");

    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    let templates_path = out_dir.join("templates.rs");
    fs::write(&templates_path, "").expect("Failed to clear templates.rs");

    embed_html_as_rust_code(&templates_dir.join("index.html"), "index_html", &out_dir);
    embed_html_as_rust_code(&templates_dir.join("404.html"), "not_found_html", &out_dir);

    println!("cargo:rerun-if-changed=templates/index.html");
    println!("cargo:rerun-if-changed=templates/404.html");
}
