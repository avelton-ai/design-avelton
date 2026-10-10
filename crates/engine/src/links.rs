//! Avelton community and project links (Help menu, About, start screen, CLI, MCP).

use serde_json::{Value, json};

/// The app's slug on avelton.online and GitHub.
pub const APP: &str = "designcraft";
/// The Avelton community Discord.
pub const DISCORD: &str = "https://github.com/avelton-ai/design-avelton/issues";
/// The Avelton website.
pub const WEBSITE: &str = "https://avelton.online";
/// Design Avelton's page on the Avelton website.
pub const APP_PAGE: &str = "https://design.avelton.online";
/// Design Avelton's source repository.
pub const GITHUB: &str = "https://github.com/avelton-ai/design-avelton";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/avelton-ai/design-avelton/issues";

/// App links accept the original command slugs and the Avelton product names.
pub fn page_of(slug: &str) -> String {
    let product = slug.strip_suffix("-avelton").or_else(|| slug.strip_suffix("craft")).unwrap_or(slug);
    format!("https://{product}.avelton.online")
}

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "discord": DISCORD,
        "website": WEBSITE,
        "appPage": APP_PAGE,
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`discord`, `website`, `appPage`, `github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "discord" => DISCORD,
        "website" => WEBSITE,
        "appPage" => APP_PAGE,
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert_eq!(APP_PAGE, "https://design.avelton.online");
        assert_eq!(GITHUB, "https://github.com/avelton-ai/design-avelton");
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["discord", "website", "appPage", "github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
    }
}
