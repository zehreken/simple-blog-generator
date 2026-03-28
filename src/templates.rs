use crate::config::SbgConfig;
use std::{fs, path::Path};

const DEFAULT_HEAD: &str = include_str!("templates/head.html");
const DEFAULT_TAG_HEAD: &str = include_str!("templates/tag_head.html");
const DEFAULT_ZETTEL: &str = include_str!("templates/zettel.html");
const DEFAULT_MAIN_CSS: &str = include_str!("templates/main.css");
const DEFAULT_ZETTEL_CSS: &str = include_str!("templates/zettelstyle.css");

pub struct ResolvedTemplates {
    pub head: String,
    pub tag_head: String,
    pub zettel: String,
    pub main_css: String,
    pub zettel_css: String,
}

pub fn resolve(project_root: &Path, config: &SbgConfig) -> ResolvedTemplates {
    let head_raw =
        try_read_override(project_root, "head.html").unwrap_or_else(|| DEFAULT_HEAD.to_string());
    let tag_head_raw = try_read_override(project_root, "tag_head.html")
        .unwrap_or_else(|| DEFAULT_TAG_HEAD.to_string());
    let zettel_raw = try_read_override(project_root, "zettel.html")
        .unwrap_or_else(|| DEFAULT_ZETTEL.to_string());
    let main_css =
        try_read_override(project_root, "main.css").unwrap_or_else(|| DEFAULT_MAIN_CSS.to_string());
    let zettel_css = try_read_override(project_root, "zettelstyle.css")
        .unwrap_or_else(|| DEFAULT_ZETTEL_CSS.to_string());

    ResolvedTemplates {
        head: apply_config(&head_raw, config),
        tag_head: apply_config(&tag_head_raw, config),
        zettel: apply_config(&zettel_raw, config),
        main_css,
        zettel_css,
    }
}

fn try_read_override(root: &Path, filename: &str) -> Option<String> {
    let path = root.join(filename);
    if path.exists() {
        println!("[templates] Using local override: {}", path.display());
        fs::read_to_string(path).ok()
    } else {
        None
    }
}

fn apply_config(template: &str, config: &SbgConfig) -> String {
    let nav_links = config
        .nav
        .as_ref()
        .map(|n| {
            n.links
                .iter()
                .map(|l| format!("<a href=\"{}\">{}</a>", l.href, l.label))
                .collect::<Vec<_>>()
                .join("\n                ")
        })
        .unwrap_or_default();

    let footer_links = config
        .footer
        .as_ref()
        .map(|f| {
            f.links
                .iter()
                .enumerate()
                .map(|(i, l)| {
                    if i == 0 {
                        format!("<a href=\"{}\">{}</a>", l.href, l.label)
                    } else {
                        format!(" | <a href=\"{}\">{}</a>", l.href, l.label)
                    }
                })
                .collect::<String>()
        })
        .unwrap_or_default();

    let analytics_block = config
        .analytics
        .as_ref()
        .and_then(|a| a.google_tag.as_deref())
        .filter(|t| !t.is_empty())
        .map(|tag| {
            format!(
                "<!-- Google Analytics -->\n    <script async src=\"https://www.googletagmanager.com/gtag/js?id={tag}\"></script>\n    <script>\n        window.dataLayer = window.dataLayer || [];\n        function gtag() {{ dataLayer.push(arguments); }}\n        gtag('js', new Date());\n        gtag('config', '{tag}');\n    </script>"
            )
        })
        .unwrap_or_default();

    let counter_block = config
        .analytics
        .as_ref()
        .and_then(|a| a.librecounter)
        .filter(|&enabled| enabled)
        .map(|_| "<img src=\"https://librecounter.org/counter.svg\" referrerPolicy=\"unsafe-url\" width=\"0\" />")
        .unwrap_or_default();

    let language = config.site.language.as_deref().unwrap_or("en_US");
    let description = config.site.description.as_deref().unwrap_or("");
    let url = config.site.url.as_deref().unwrap_or("/");

    template
        .replace("$site_name", &config.site.name)
        .replace("$language", language)
        .replace("$description", description)
        .replace("$site_url", url)
        .replace("$analytics_block", &analytics_block)
        .replace("$counter_block", counter_block)
        .replace("$nav_links", &nav_links)
        .replace("$footer_links", &footer_links)
}
