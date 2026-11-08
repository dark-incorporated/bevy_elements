//! **No WESL file in this crate declares the same function twice.** A duplicate is a shader that
//! fails to compile at runtime, on the machine of whoever enabled the feature — not here.
//!
//! This is not hypothetical. `src/noise.wesl` shipped `perlin3_periodic` **twice**, byte for byte
//! identical, after a rebase in 2026-11-03 duplicated the block. Nothing in the build notices: WESL
//! is composed and compiled by `naga` when a pipeline is first used, so `cargo build`, `cargo test`
//! and CI all stayed green, and the failure surfaced only when a consumer (bevy_dark_factory's editor)
//! opened a window with `bevy_elements` enabled and got
//!
//! ```text
//! ERROR bevy_render::render_resource::pipeline_cache: failed to process shader error:
//! error: duplicate declaration of `perlin3_periodic`
//! ```
//!
//! every frame, on a viewport that drew nothing usable. So the check is a source scan, in the
//! precedent of this repo's other ratchets: cheap, deterministic, no GPU, and it names the file and
//! the symbol when it fails.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn wesl_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            wesl_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "wesl" || e == "wgsl") {
            out.push(path);
        }
    }
}

/// `fn name(` at the start of a line, which is how every function in these files is declared.
///
/// **A function inside an `@if`/`@else` branch is not counted.** WESL's conditional compilation keeps
/// exactly one branch, and `terrain/render.wesl` uses the idiom deliberately:
///
/// ```text
/// @if(GROUND_COLOURS)  fn sand_colour() -> vec3<f32> { … }   @else  fn sand_colour() -> vec3<f32> { … }
/// ```
///
/// so flagging it would be a false positive on the crate's own design. A declaration is skipped while
/// the file is inside a conditional run — one opened by a line starting `@if(` or `@else`, and closed
/// by a line whose only content is `}`.
fn declared_functions(text: &str) -> Vec<(String, usize)> {
    let mut found = Vec::new();
    let mut depth: usize = 0;
    let mut open_braces: usize = 0;
    for (ix, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("@if(") {
            if depth == 0 {
                open_braces = 0;
            }
            depth += 1;
            continue;
        }
        if trimmed.starts_with("@else") {
            continue;
        }
        if depth > 0 {
            // Track brace balance from the branch's opening line so the block's closing `}` returns
            // us to normal code, while `@group(...)` attribute lines inside a branch are ignored.
            open_braces += line.matches('{').count();
            open_braces = open_braces.saturating_sub(line.matches('}').count());
            if open_braces == 0 && trimmed == "}" {
                depth -= 1;
            }
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("fn ") else { continue };
        let Some((name, _)) = rest.split_once('(') else { continue };
        let name = name.trim();
        if !name.is_empty() {
            found.push((name.to_owned(), ix + 1));
        }
    }
    found
}

/// **One declaration per name, per file.** WESL composes modules by import, so two *different* files
/// may each have a private `lerp`; two declarations in one file is always a mistake, and it is the
/// one this crate shipped.
#[test]
fn no_file_declares_the_same_function_twice() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    wesl_files(&root, &mut files);
    files.sort();
    assert!(!files.is_empty(), "no shader files found under {} — the scan is broken", root.display());

    let mut problems: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for path in &files {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        let mut seen: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (name, line) in declared_functions(&text) {
            seen.entry(name).or_default().push(line);
        }
        for (name, lines) in seen {
            if lines.len() > 1 {
                let relative = path.strip_prefix(&root).unwrap_or(path).display().to_string();
                problems.insert(format!("{relative}: fn {name}"), lines);
            }
        }
    }
    assert!(
        problems.is_empty(),
        "a shader declares the same function twice, which `naga` refuses at pipeline-build time — \
         on the consumer's machine, not in this build:\n{}",
        problems
            .iter()
            .map(|(what, lines)| format!("  {what} at lines {lines:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
