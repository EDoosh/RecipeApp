use crate::v1::types::database::{Method, Quiz, Recipe as DatabaseRecipe};
use crate::v1::types::*;

/// A recipe that contains less information than a standard `Recipe` or
/// a database Recipe. This is used to reduce the amount of data that is
/// sent to the client.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    /// The unique identifier of the recipe.
    pub uuid: Uuid,
    /// The date the recipe was made public.
    pub date_added: Date,
    /// Whether the recipe is *currently* the weekly recipe.
    pub is_weekly: bool,
    /// The short name of the recipe, used in the URL.
    ///
    /// e.g., `/recipe/chicken-tikka-masala`, `chicken-tikka-masala` is this string.
    pub short: String,
    /// The title of the recipe.
    pub title: String,
    /// The nutrients found in the recipe.
    pub nutrients: Vec<SerdeStringNutrient>,
    /// The amount of time, in minutes, to cook the recipe.
    pub time_to_cook: u16,
    /// The number of servings the recipe makes.
    pub servings: u16,
    /// The URL to the image of the recipe.
    pub image: Url,
    /// The gradient of the recipe.
    pub gradient: Gradient,
    /// The ingredients of the recipe.
    pub ingredients: Vec<String>,
    /// The recipe's method
    pub method: Method,
    /// The recipe's quiz.
    pub quiz: Quiz,
}

impl Recipe {
    /// Creates a new `Recipe` from a [`database::Recipe`].
    pub async fn from_recipe(
        recipe: &DatabaseRecipe,
        weekly_getter: &crate::WeeklyRecipeGetter,
    ) -> Self {
        Recipe {
            uuid: recipe.uuid,
            // Return the date it became public instead of the date it
            // was added to the database
            date_added: recipe.becomes_public,
            is_weekly: recipe.is_weekly(weekly_getter).await,
            short: recipe.short.clone(),
            title: recipe.title.clone(),
            // Convert Nutrient to SerdeStringNutrient so when sent to the
            // client it will be serialized as a string.
            nutrients: recipe.nutrients.iter().map(|&n| n.into()).collect(),
            time_to_cook: recipe.time_to_cook,
            servings: recipe.servings,
            image: recipe.image.clone(),
            gradient: recipe.gradient.clone(),
            ingredients: recipe.ingredients.clone(),
            method: recipe.method.clone(),
            quiz: recipe.quiz.clone(),
        }
    }
}
