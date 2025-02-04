use crate::HTTP::Client;
use crate::errors::ModPlatformsErrors;

use super::MODRINTH_API;
use serde::{Deserialize, Serialize};

/// Represents a complete Modrinth project entity containing all metadata and relationships.
#[derive(Deserialize, Debug)]
pub struct ModrinthProject {
    /// The display title of the project.
    pub title: String,
    
    /// Detailed description of the project.
    pub description: String,
    
    /// Primary categories the project belongs to.
    pub categories: Vec<String>,
    
    /// Indicates client-side compatibility requirements.
    pub client_side: String,
    
    /// Indicates server-side compatibility requirements.
    pub server_side: String,
    
    /// Main content body of the project description.
    pub body: String,
    
    /// Current approval status of the project.
    pub status: String,
    
    /// Additional categorizations beyond primary categories.
    pub additional_categories: Vec<String>,
    
    /// URL for reporting issues related to the project.
    pub issues_url: Option<String>,
    
    /// Source code repository URL if available.
    pub source_url: Option<String>,
    
    /// Documentation/wiki URL for the project.
    pub wiki_url: Option<String>,
    
    /// Discord community URL for the project.
    pub discord_url: Option<String>,
    
    /// Collection of donation URLs supporting the project.
    pub donation_urls: Vec<DonationUrl>,
    
    /// Type classification of the project (mod, resourcepack, etc.).
    pub project_type: String,
    
    /// Total number of downloads for the project.
    pub downloads: i64,
    
    /// URL pointing to the project's icon/image.
    pub icon_url: String,
    
    /// Timestamp when the project was first published.
    pub published: String,
    
    /// Timestamp of the most recent update.
    pub updated: String,
    
    /// Timestamp when the project was approved.
    pub approved: String,
    
    /// Number of users following the project.
    pub followers: i64,
    
    /// Licensing information for the project.
    pub license: License,
    
    /// List of all versions available for the project.
    pub versions: Vec<String>,
    
    /// Compatible Minecraft game versions.
    pub game_versions: Vec<String>,
    
    /// Supported mod loaders for installation.
    pub loaders: Vec<String>,
    
    /// Optional direct link to the project body content.
    pub body_url: Option<String>,
    
    /// Indicates if the project is queued for moderation.
    pub queued: Option<bool>,
    
    /// Message from moderators regarding the project.
    pub moderator_message: Option<String>,
}

/// Represents licensing information for a project.
#[derive(Deserialize, Debug)]
pub struct License {
    /// Unique identifier for the license.
    pub id: String,
    
    /// Human-readable name of the license.
    pub name: String,
    
    /// Optional URL linking to the full license text or details.
    pub url: Option<String>,
}

/// Represents a single donation URL endpoint for supporting a project.
#[derive(Serialize, Deserialize, Debug)]
pub struct DonationUrl {
    /// Unique identifier for this donation platform.
    pub id: String,
    
    /// Name of the donation platform (e.g., Patreon, Ko-fi).
    pub platform: String,
    
    /// Direct URL where users can donate to support the project.
    pub url: String,
}


/// Get a single project from modrinth.
pub async fn get_project(
    client: Client,
    project: &str,
    user_agent: &str,
) -> Result<ModrinthProject, ModPlatformsErrors> {
    let url = format!("{}/project/{}", MODRINTH_API, project);
    let res = client
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .map_err(|err| {
            ModPlatformsErrors::RequestError(format!("Failed to request to modrinth: {}", err.to_string())) 
        })?;

    let modrinth_project: ModrinthProject = res.json().await.map_err(|err| {
        ModPlatformsErrors::DeserializationError(format!(
            "Failed to deserialize modrinth error: {}",
            err
        ))
    })?;

    Ok(modrinth_project)
}


pub async fn get_multiple_projects(client: Client, user_agent: &str, ) -> Result<(), ModPlatformsErrors> {
    let mut params: Vec<String> = vec![];
    let url = format!("{}/search", MODRINTH_API);
    // Maxium Limit is 100.

    Ok(())
}
