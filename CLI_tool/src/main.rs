// long version cargo run -- --input sample.md --output xyz.html
//Short version cargo run -- -i sample.md -o xyz.html
use clap::Parser; 
use maud::{Markup, PreEscaped, html};
use pulldown_cmark::{Options, Parser as MarkdownParser, html};
use std::{fs, path::PathBuf};
#[derive(Parser, Debug)]
struct Arg {
    //Input markdown file path
    #[arg(long, short)]
    input:PathBuf,
    //Ouput html file path
    #[arg(long, short)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    html! {
          (maud::DOCTYPE)
        html lang="en" {
        head {
            meta charset="utf-8";
            title {"Markdown to HTML Output"}
        }
         style {
                    r#"
                    body {
                        font-family: Arial, sans-serif;
                        max-width: 850px;
                        margin: 2rem auto;
                        padding: 1.5rem;
                        background: #fafafa;
                        color: #222;
                        line-height: 1.7;
                    }
                    h1, h2, h3 { color: #333; }
                    code {
                        background: #eee;
                        padding: 2px 5px;
                        border-radius: 6px;
                        font-size: 0.95em;
                    }
                    pre code {
                        display: block;
                        padding: 1rem;
                    }
                    blockquote {
                        border-left: 4px solid #ccc;
                        padding-left: 12px;
                        color: #555;
                        font-style: italic;
                    }
                    .dark-mode {
                       background: #121212;
                       color: #e6e6e6;
                    }                 
                    "#
                }
body {
    (maud::PreEscaped(content.to_string()))

    hr {}
    h3 { "Thank you for checking out my project! 😄" }

    button id="theme-toggle" { "🌗 Theme" }

    script {
        r#"
        const body = document.body;
        const saved = localStorage.getItem("theme");
        if (saved === "dark" ||
            (!saved && window.matchMedia("(prefers-color-scheme: dark)").matches)) {
            body.classList.add("dark-mode");
        }
        document.getElementById("theme-toggle").onclick = () => {
            body.classList.toggle("dark-mode");
            localStorage.setItem("theme",
                body.classList.contains("dark-mode") ? "dark" : "light");
        };
        "#
    }
}
    }
}
}

fn main() {
    let args = Arg::parse();
    let markdown_input = fs::read_to_string(&args.input)
    .expect("Fail to read ");

let mut options = Options::empty();
options.insert(Options::ENABLE_STRIKETHROUGH);

let parser = MarkdownParser::new_ext(&markdown_input, options);
let mut html_output =String::new();

html::push_html(&mut html_output, parser);
let full_html_output = render_html_page(&html_output).into_string();

match args.output{
    Some(path)=>{ fs::write(path,full_html_output)
    .expect("Failed to write");
println!("HTML file generated successfully!")
}
    None => println!("{}", full_html_output),
}
}
