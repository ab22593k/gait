use handlebars::Handlebars;
use serde::Serialize;
use std::path::Path;
use std::sync::OnceLock;
use git2::Repository;

static TEMPLATE_ENGINE: OnceLock<Handlebars<'static>> = OnceLock::new();

/// Get the project root directory (git repository root)
fn get_project_root() -> anyhow::Result<std::path::PathBuf> {
    let repo = Repository::discover(".")?;
    Ok(repo.workdir().unwrap_or(repo.path().parent().unwrap()).to_path_buf())
}

/// Initialize the global template engine with templates from the templates directory
pub fn init_templates() -> anyhow::Result<()> {
    let mut handlebars = Handlebars::new();

    // Set up template directory
    let project_root = get_project_root()?;
    let templates_dir = project_root.join("templates");

    if !templates_dir.exists() {
        return Err(anyhow::anyhow!("Templates directory not found: {}", templates_dir.display()));
    }

    // Load all .hbs files from templates directory
    load_templates_from_dir(&mut handlebars, &templates_dir)?;

    // Register the engine
    TEMPLATE_ENGINE.set(handlebars).map_err(|_| {
        anyhow::anyhow!("Template engine already initialized")
    })?;

    Ok(())
}

/// Get the global template engine instance, initializing if necessary
pub fn get_template_engine() -> &'static Handlebars<'static> {
    if TEMPLATE_ENGINE.get().is_none() {
        init_templates().expect("Failed to initialize templates");
    }
    TEMPLATE_ENGINE.get().unwrap()
}

/// Render a template with the given data
pub fn render_template<T: Serialize>(template_name: &str, data: &T) -> anyhow::Result<String> {
    let engine = get_template_engine();
    engine.render(template_name, data)
        .map_err(|e| anyhow::anyhow!("Failed to render template '{}': {}", template_name, e))
}

/// Load all .hbs templates from a directory recursively
fn load_templates_from_dir(handlebars: &mut Handlebars<'static>, dir: &Path) -> anyhow::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            load_templates_from_dir(handlebars, &path)?;
        } else if let Some(extension) = path.extension() {
            if extension == "hbs" {
                let template_name = path.strip_prefix(dir)?
                    .with_extension("")
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/");

                let content = std::fs::read_to_string(&path)?;
                handlebars.register_template_string(&template_name, content)?;
            }
        }
    }

    Ok(())
}

/// Template context data structures
#[derive(Serialize)]
pub struct CommitSystemTemplateContext<'a> {
    pub config: &'a crate::config::Config,
    pub schema: serde_json::Value,
    pub combined_instructions: String,
}

#[derive(Serialize)]
pub struct CommitUserTemplateContext<'a> {
    pub context: &'a crate::core::context::CommitContext,
    pub recent_commits: String,
    pub staged_changes: String,
    pub project_metadata: String,
    pub detailed_changes: String,
    pub author_history: String,
}

#[derive(Serialize)]
pub struct ReviewTemplateContext<'a> {
    pub config: &'a crate::config::Config,
    pub context: &'a crate::core::context::CommitContext,
    pub schema: serde_json::Value,
    pub combined_instructions: String,
    pub dimensions_descriptions: String,
    pub dimensions_json: String,
}

#[derive(Serialize)]
pub struct PrTemplateContext<'a> {
    pub config: &'a crate::config::Config,
    pub context: &'a crate::core::context::CommitContext,
    pub commit_messages: &'a [String],
    pub schema: serde_json::Value,
    pub combined_instructions: String,
    pub commits_section: String,
    pub recent_commits: String,
    pub staged_changes: String,
    pub project_metadata: String,
    pub detailed_changes: String,
}