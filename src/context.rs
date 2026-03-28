use crate::{
    config::SbgConfig,
    templates::{self, ResolvedTemplates},
};
use std::path::{Path, PathBuf};

pub struct BuildContext {
    pub output_dir: PathBuf,
    pub posts_dir: PathBuf,
    pub zettelkasten_dir: PathBuf,
    #[allow(dead_code)]
    pub config: SbgConfig,
    pub templates: ResolvedTemplates,
}

impl BuildContext {
    pub fn new(project_root: &Path, config: SbgConfig, output_override: Option<PathBuf>) -> Self {
        let output_dir = output_override.unwrap_or_else(|| {
            let dir = config
                .paths
                .as_ref()
                .and_then(|p| p.output.as_deref())
                .unwrap_or("docs");
            project_root.join(dir)
        });

        let posts_dir = project_root.join(
            config
                .paths
                .as_ref()
                .and_then(|p| p.posts.as_deref())
                .unwrap_or("source/posts"),
        );

        let zettelkasten_dir = project_root.join(
            config
                .paths
                .as_ref()
                .and_then(|p| p.zettelkasten.as_deref())
                .unwrap_or("source/zettelkasten"),
        );

        let templates = templates::resolve(project_root, &config);

        BuildContext {
            output_dir,
            posts_dir,
            zettelkasten_dir,
            config,
            templates,
        }
    }
}
