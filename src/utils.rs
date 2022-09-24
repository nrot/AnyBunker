use sea_query::{TableRef, Alias};
use syntect::highlighting::ThemeSet;
use syntect::html::{ClassedHTMLGenerator, ClassStyle, css_for_theme_with_class_style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use std::sync::Arc;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::credentials;

#[deprecated]
#[inline(always)]
pub fn json_html_syntect(s: String) -> String {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    // let ts = ThemeSet::load_defaults();
    // let mut theme = &ts.themes["base16-ocean.light"];
    // let c = theme.settings.background.unwrap_or(Color::WHITE);
    let syntax = ps.find_syntax_by_extension("json").unwrap();
    // let html = highlighted_html_for_string(&s, &ps, syntax, &theme).unwrap();

    let mut h = ClassedHTMLGenerator::new_with_class_style(syntax, &ps, ClassStyle::Spaced);
    for line in LinesWithEndings::from(&s) {
        h.parse_html_for_line_which_includes_newline(line).unwrap();
    }
    h.finalize()
}



pub fn generate_styles(){
    let ts = ThemeSet::load_defaults();

    // create dark color scheme css
    let dark_theme = &ts.themes["Solarized (dark)"];
    let css_dark_file = File::create(Path::new("theme-dark.css")).unwrap();
    let mut css_dark_writer = BufWriter::new(&css_dark_file);

    let css_dark = css_for_theme_with_class_style(dark_theme, ClassStyle::Spaced).unwrap();
    writeln!(css_dark_writer, "{}", css_dark).unwrap();

    // create light color scheme css
    let light_theme = &ts.themes["Solarized (light)"];
    let css_light_file = File::create(Path::new("theme-light.css")).unwrap();
    let mut css_light_writer = BufWriter::new(&css_light_file);

    let css_light = css_for_theme_with_class_style(light_theme, ClassStyle::Spaced).unwrap();
    writeln!(css_light_writer, "{}", css_light).unwrap();
}

pub fn with_schema(t: &str)->TableRef{
    TableRef::SchemaTable(Arc::new(Alias::new(&credentials::postgres_schema())), Arc::new(Alias::new(t)))
}