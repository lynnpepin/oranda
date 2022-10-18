use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::errors::*;

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::{self, ComrakOptions, ComrakPlugins};

mod syntax_highlight;
use syntax_highlight::syntax_highlight;

pub struct MockAdapter {}
impl SyntaxHighlighterAdapter for MockAdapter {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        let highlighted_code = syntax_highlight(lang, code);

        // requires a string to be returned
        match highlighted_code {
            Ok(code) => code,
            Err(_) => String::from(""),
        }
    }

    fn build_pre_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::from("")
    }

    fn build_code_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::from("")
    }
}

fn initialize_comrak_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();

    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;

    options
}

fn load(readme_path: &Path) -> Result<String> {
    let readme = fs::read_to_string(readme_path)?;
    Ok(readme)
}

pub fn body(readme_path: &Path) -> Result<String> {
    let readme = load(readme_path)?;
    let options = initialize_comrak_options();

    let mut plugins = ComrakPlugins::default();
    let adapter = MockAdapter {};
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    Ok(comrak::markdown_to_html_with_plugins(
        &readme, &options, &plugins,
    ))
}