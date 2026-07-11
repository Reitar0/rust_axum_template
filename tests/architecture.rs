//! Архитектурный тест: бизнес-домены изолированы друг от друга.
//!
//! Файлы одного домена (`src/domains/<X>/`) не должны ссылаться на код другого
//! домена (`crate::domains::<Y>`). Общее — только через `shared`. Так «мягкая»
//! граница по соглашению превращается в проверяемую компилятором тестов.

use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn domains_are_isolated() {
    let domains_dir = Path::new("src/domains");
    if !domains_dir.exists() {
        return;
    }

    // Домены = подпапки в src/domains/.
    let domains: Vec<String> = fs::read_dir(domains_dir)
        .expect("не удалось прочитать src/domains")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    let mut violations = Vec::new();
    for domain in &domains {
        for file in rs_files(&domains_dir.join(domain)) {
            let content = fs::read_to_string(&file).expect("не удалось прочитать файл домена");
            for other in &domains {
                if other == domain {
                    continue;
                }
                if content.contains(&format!("crate::domains::{other}")) {
                    violations.push(format!(
                        "домен `{domain}` использует код чужого домена `{other}` в {}",
                        file.display()
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Нарушения изоляции доменов (используй shared вместо чужого домена):\n{}",
        violations.join("\n")
    );
}

/// Рекурсивно собирает все `.rs` файлы в каталоге.
fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(rs_files(&path));
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    out
}
