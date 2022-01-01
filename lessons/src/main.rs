// Here we're importing some crates and some modules, and also
// exposing some types from those modules into the current scope.
use pulldown_cmark::{html, Options, Parser};
use std::{
    env,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
    str,
};

// This is a constant. Constants can be used in pattern matches!
const MARKDOWN_FILE_EXTENSION: &str = "md";
const SRC_DIR: &str = "src";
const SECTIONS_DIR: &str = "sections";
const TARGET_DIR: &str = "target";
const HTML_DIR: &str = "html-lessons";
const BASE_TITLE: &str = "Introduction to Rust";

#[derive(Debug)]
struct Section {
    name: String,
    subsections: Vec<Subsection>,
}

#[derive(Debug)]
struct Subsection {
    index: u16,
    name: String,
    path: PathBuf,
}

fn main() {
    // The unwrap() method on a `Result` basically turns an Err into a panic,
    // and an Ok into the value contained in the Ok. This is lazy error
    // handling (I'd usually rather gracefully handle an Err) but this is,
    // after all, only something I'm writing for my own use!
    //
    // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
    run().unwrap();
}

// io::Result<T> is an alias for Result<T, io::Error> - you can also write
// a normal Result here and explicitly choose io::Error for the error type,
// but using the io::Result shorthand is more common.
//
// https://doc.rust-lang.org/std/io/type.Result.html
// https://doc.rust-lang.org/std/io/struct.Error.html
fn run() -> io::Result<()> {
    // Here's the official documentation for the ? operator:
    //
    // https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html
    let sections_dir = env::current_dir()?.join(SRC_DIR).join(SECTIONS_DIR);

    // Gather up all the section_dirs
    let sections = {
        let mut section_dirs = Vec::new();

        for section_entry in fs::read_dir(&sections_dir)? {
            let section_entry = section_entry?;

            if section_entry.file_type()?.is_dir() {
                let path = section_entry.path();
                let section_name = path.file_stem().unwrap().to_str().unwrap();

                // Convert from `&Path` to `PathBuf` to prevent a lifetime error.
                section_dirs.push((section_name.to_string(), path.to_path_buf()));
            }
        }

        section_dirs.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));

        populate_sections(section_dirs.as_slice())?
    };

    // Build the sidebar
    let sidebar_html = {
        let mut buffer = "<nav id=\"sidebar-nav\">\n<ol class=\"lesson-links\">".to_string();

        for (section_index, section) in sections.iter().enumerate() {
            buffer.push_str("<li>\n");

            buffer.push_str(&format!("<h4>{}</h4>\n", section.name));

            for (subsection_index, subsection) in section.subsections.iter().enumerate() {
                let url = format!("/{}.{}", section_index + 1, subsection_index + 1);

                buffer.push_str(&format!(
                    "\t<a class=\"subsection\" href=\"{}\">{}</a>\n",
                    url, subsection.name
                ));
            }

            buffer.push_str("</li>\n");
        }

        buffer.push_str("<ol>\n</nav>\n");

        buffer
    };

    // e.g. `./target/html-lessons/`
    let html_dir = env::current_dir()?.join(TARGET_DIR).join(HTML_DIR);
    fs::create_dir_all(&html_dir)?;

    write_and_log(&html_dir.join("styles.css"), include_str!("styles.css"))?;
    write_and_log(&html_dir.join("favicon.svg"), include_str!("favicon.svg"))?;

    // Write the welcome page
    {
        let html = md_to_html(BASE_TITLE, &sections_dir.join("welcome.md"), &sidebar_html)?;

        write_and_log(&html_dir.join("index.html"), &html)?;
    }

    // Build the .html files
    {
        for (section_index, section) in sections.iter().enumerate() {
            for (subsection_index, subsection) in section.subsections.iter().enumerate() {
                let dir_name = format!("{}.{}", section_index + 1, subsection_index + 1);

                let html = md_to_html(
                    &format!("{} - {}", subsection.name, BASE_TITLE),
                    &subsection.path,
                    &sidebar_html,
                )?;

                // e.g. `./target/html-lessons/1.1/`
                let target_dir = html_dir.join(&dir_name);

                // e.g. `./target/html-lessons/1.1/index.html`
                let target_file = target_dir.join("index.html");

                // Make sure the target directory exists, with the equivalent of `mkdir -p`
                fs::create_dir_all(target_dir)?;

                write_and_log(&target_file, &html)?;
            }
        }
    };

    Ok(())
}

fn write_and_log(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)?;

    println!("Wrote {:?}", path);

    Ok(())
}

fn populate_sections(section_dirs: &[(String, PathBuf)]) -> io::Result<Vec<Section>> {
    let mut sections = Vec::new();

    for (section_name, section_path) in section_dirs {
        // Strip off the number-plus-underscore prefix, e.g. "1_"
        let section_name = section_name.split('_').last().unwrap();
        let mut subsections = Vec::new();

        for section_entry in fs::read_dir(section_path)? {
            let path = section_entry?.path();

            // `if let` is basically a `match` with a single branch. If this pattern
            // matches, then we run the branch; otherwise, we don't.
            if let Some(os_ext) = path.extension() {
                if let Some(MARKDOWN_FILE_EXTENSION) = os_ext.to_str() {
                    // Strip off the number-plus-underscore prefix, e.g. "1_"
                    let mut split = path.file_stem().unwrap().to_str().unwrap().split('_');
                    let index = split.next().unwrap().parse::<u16>().unwrap();
                    let subsection_name = split.next().unwrap().to_string();

                    // Convert from `&Path` to `PathBuf` to prevent a lifetime error.
                    subsections.push(Subsection {
                        index,
                        name: subsection_name,
                        path: path.to_path_buf(),
                    });
                }
            }
        }

        subsections.sort_by(|sub1, sub2| sub1.index.cmp(&sub2.index));

        sections.push(Section {
            name: section_name.to_string(),
            subsections,
        });
    }

    Ok(sections)
}

fn md_to_html(title: &str, path: &Path, sidebar_html: &str) -> io::Result<String> {
    let header_html = include_str!("header.html");
    let footer_html = include_str!("footer.html");
    let before_title_html = include_str!("before_title.html");
    let before_body_html = include_str!("before_body.html");
    let after_body_html = include_str!("after_body.html");

    let mut buffer = format!(
        "{}{}{}{}{}\n<main>",
        before_title_html, title, before_body_html, header_html, sidebar_html
    );

    // Read the contents of the .md file
    let md = {
        let mut buffer = String::with_capacity(10_000);

        File::open(path)?.read_to_string(&mut buffer).unwrap();

        buffer
    };

    // Convert to HTML and write that HTML to the file.
    let parser = {
        let mut options = Options::empty();

        options.insert(Options::ENABLE_STRIKETHROUGH);

        Parser::new_ext(&md, options)
    };

    html::push_html(&mut buffer, parser);

    buffer.push_str(&format!("</main>\n{}{}", footer_html, after_body_html));

    Ok(buffer)
}
