use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub title: String,
    pub description: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Recipe {
    pub fn new(
        title: String,
        description: String,
        ingredients: Vec<String>,
        instructions: Vec<String>,
        tags: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        let namespace = Uuid::NAMESPACE_DNS;
        let content = format!("{}|{}", title, description);
        let id = Uuid::new_v5(&namespace, content.as_bytes()).to_string();
        Self {
            id,
            title,
            description,
            ingredients,
            instructions,
            tags,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", self.title));
        md.push_str(&format!("{}\n\n", self.description));
        md.push_str("## Ingredients\n\n");
        for ingredient in &self.ingredients {
            md.push_str(&format!("- {}\n", ingredient));
        }
        md.push_str("\n## Instructions\n\n");
        for (i, instruction) in self.instructions.iter().enumerate() {
            md.push_str(&format!("{}. {}\n", i + 1, instruction));
        }
        if !self.tags.is_empty() {
            md.push_str("\n## Tags\n\n");
            md.push_str(&format!("Tags: {}\n", self.tags.join(", ")));
        }
        md.push_str(&format!(
            "\n---\n*Created: {}*\n",
            self.created_at.format("%Y-%m-%d")
        ));
        md
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipeRequest {
    pub title: String,
    pub description: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub tags: Vec<String>,
}

pub type RecipeStore = std::sync::Arc<tokio::sync::RwLock<Vec<Recipe>>>;

pub async fn load_recipes_from_disk(store: &RecipeStore) {
    let recipes_dir = Path::new("recipes");
    
    if !recipes_dir.exists() {
        fs::create_dir_all(recipes_dir).ok();
        add_sample_recipes(store).await;
        return;
    }

    let mut recipes = Vec::new();
    
    if let Ok(entries) = fs::read_dir(recipes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(recipe) = serde_json::from_str::<Recipe>(&contents) {
                        recipes.push(recipe);
                    }
                }
            }
        }
    }

    let mut store_recipes = store.write().await;
    if recipes.is_empty() {
        add_sample_recipes(store).await;
    } else {
        *store_recipes = recipes;
    }
}

pub async fn add_sample_recipes(store: &RecipeStore) {
    let mut recipes = store.write().await;

    let sample1 = Recipe::new(
        "Classic Grilled Cheese".to_string(),
        "A simple but perfect grilled cheese sandwich".to_string(),
        vec![
            "2 slices bread".to_string(),
            "2 slices cheddar cheese".to_string(),
            "2 tbsp butter".to_string(),
        ],
        vec![
            "Butter one side of each bread slice".to_string(),
            "Place cheese between unbuttered sides".to_string(),
            "Cook on medium heat for 3-4 minutes each side until golden brown".to_string(),
        ],
        vec![
            "comfort food".to_string(),
            "quick".to_string(),
            "vegetarian".to_string(),
        ],
    );
    
    let json1 = sample1.to_json().unwrap();
    let filename1 = format!("{}-{}.json", 
        sample1.title.to_lowercase().replace(' ', "-"), 
        &sample1.id[..8]
    );
    fs::write(Path::new("recipes").join(&filename1), &json1).ok();
    recipes.push(sample1);

    let sample2 = Recipe::new(
        "Perfect Fried Egg".to_string(),
        "The foolproof method for a perfectly fried egg".to_string(),
        vec![
            "1 egg".to_string(),
            "1 tbsp butter or oil".to_string(),
            "Salt and pepper to taste".to_string(),
        ],
        vec![
            "Heat butter in non-stick pan over medium-low heat".to_string(),
            "Crack egg into pan".to_string(),
            "Cook until white is set and yolk is desired consistency".to_string(),
            "Season with salt and pepper".to_string(),
        ],
        vec![
            "breakfast".to_string(),
            "quick".to_string(),
            "easy".to_string(),
        ],
    );
    
    let json2 = sample2.to_json().unwrap();
    let filename2 = format!("{}-{}.json", 
        sample2.title.to_lowercase().replace(' ', "-"), 
        &sample2.id[..8]
    );
    fs::write(Path::new("recipes").join(&filename2), &json2).ok();
    recipes.push(sample2);
}
