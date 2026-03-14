use crate::{
    structs::{ManifestVersion, VersionManifest},
    MANIFEST_URL,
    HTTP::{header::USER_AGENT, Client}
};

pub async fn get_version_manifest(
    client: Client,
    url: &str,
    user_agent: &str,
) -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let result = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let version_manifest: VersionManifest = result.json().await?;

    Ok(version_manifest)
}

pub async fn get_manifest(client: Client, user_agent: &str) -> Result<ManifestVersion, Box<dyn std::error::Error>> {
    let result = client
        .get(MANIFEST_URL)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let manifest: ManifestVersion = result.json().await?;

    Ok(manifest)
}



pub async setup_minecraft() {

}
