pub fn to_html(markdown: &str) -> String {
    use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    // Render code blocks (```lang ... ```) as <pre class="prettyprint linenums">
    // so they get the same styling as hand-written <pre> blocks, while the
    // contents are HTML-escaped (safe for C++ templates, #include <...>, etc.).
    let mut in_code_block = false;
    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        Event::Start(Tag::CodeBlock(kind)) => {
            in_code_block = true;
            let lang = match kind {
                CodeBlockKind::Fenced(info) => info
                    .split_whitespace()
                    .next()
                    .map(|l| l.to_string())
                    .unwrap_or_default(),
                CodeBlockKind::Indented => String::new(),
            };
            let class = if lang.is_empty() {
                "prettyprint linenums".to_string()
            } else {
                let mut escaped = String::new();
                let _ = pulldown_cmark::escape::escape_html(&mut escaped, &lang);
                format!("prettyprint linenums lang-{}", escaped)
            };
            Event::Html(format!("<pre class=\"{}\">", class).into())
        }
        Event::End(Tag::CodeBlock(_)) => {
            in_code_block = false;
            Event::Html("</pre>\n".into())
        }
        Event::Text(text) if in_code_block => {
            let mut escaped = String::new();
            let _ = pulldown_cmark::escape::escape_html(&mut escaped, &text);
            Event::Html(escaped.into())
        }
        other => other,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[cfg(test)]
mod tests {
    use super::to_html;

    #[test]
    fn fenced_code_uses_prettyprint_and_escapes() {
        let html = to_html("```cpp\n#include <vector>\nstd::vector<int> v;\n```\n");
        assert_eq!(
            html,
            "<pre class=\"prettyprint linenums lang-cpp\">#include &lt;vector&gt;\nstd::vector&lt;int&gt; v;\n</pre>\n"
        );
    }

    #[test]
    fn fenced_code_without_lang() {
        let html = to_html("```\na < b\n```\n");
        assert_eq!(html, "<pre class=\"prettyprint linenums\">a &lt; b\n</pre>\n");
    }
}
