use std::env;

pub struct GitHubConfig {
    pub owner: String,
    pub repo: String,
}

impl Default for GitHubConfig {
    fn default() -> Self {
        Self {
            owner: env::var("GITHUB_OWNER").unwrap_or_else(|_| "dev-nolant".to_string()),
            repo: env::var("GITHUB_REPO").unwrap_or_else(|_| "gitcooked".to_string()),
        }
    }
}

impl GitHubConfig {
    pub fn issue_url(&self, title: &str, body: &str) -> String {
        format!(
            "https://github.com/{}/{}/issues/new?title={}&body={}",
            self.owner,
            self.repo,
            urlencoding::encode(title),
            urlencoding::encode(body)
        )
    }
}

pub fn format_recipe_issue(recipe: &crate::models::CreateRecipeRequest) -> String {
    let mut body = format!("## New Recipe: {}\n\n", recipe.title);
    body.push_str(&format!("**Description:** {}\n\n", recipe.description));
    body.push_str("### Ingredients\n\n");
    for ingredient in &recipe.ingredients {
        body.push_str(&format!("- {}\n", ingredient));
    }
    body.push_str("\n### Instructions\n\n");
    for (i, instruction) in recipe.instructions.iter().enumerate() {
        body.push_str(&format!("{}. {}\n", i + 1, instruction));
    }
    if !recipe.tags.is_empty() {
        body.push_str(&format!("\n### Tags\n\n{}\n", recipe.tags.join(", ")));
    }
    body.push_str("\n---\n");
    body.push_str("Please review and add this recipe to the `/recipes/` directory as a JSON file.");
    body
}
