use tauri::{command, AppHandle, Runtime};
use tauri::{Url, WebviewWindowBuilder};

use minecraft_essentials::{AuthType, AuthenticationBuilder, CustomAuthData};
#[cfg(feature = "open")]
use open;

use std::str::FromStr;

#[command]
#[cfg(desktop)]
pub async fn oauth<R: Runtime>(
    app: AppHandle<R>,
    client_id: &str,
    client_secret: &str,
    bedrock_rel: bool,
    refresh_token: Option<String>,
    scope: Option<String>,
    port: Option<u16>,
) -> Result<CustomAuthData, String> {
    let mut builder = AuthenticationBuilder::builder();

    if let Some(token) = refresh_token {
        let _ = builder
            .of_type(AuthType::Refresh)
            .refresh_token(Some(token))
            .map_err(|e| e.to_string())?;
    } else {
        builder.of_type(AuthType::Oauth);
    }

    if scope.is_some() {
        builder.scope(scope);
    };

    builder
        .client_id(client_id)
        .client_secret(client_secret)
        .bedrockrel(bedrock_rel)
        .port(port);

    let get_info = builder.get_info().await;
    let oauth_url_str = get_info.ouath_url.ok_or_else(|| {
        "Could not retrieve the OAuth URL from the authentication provider.".to_string()
    })?;

    let webview_window_to_close: Option<tauri::WebviewWindow<R>> = {
        #[cfg(not(feature = "open"))]
        {
            let url = Url::from_str(&oauth_url_str).map_err(|e| e.to_string())?;

            let window = WebviewWindowBuilder::new(
                &app,
                "minecraft-authentication",
                tauri::WebviewUrl::External(url),
            )
            .title("Minecraft Authentication")
            .closable(true)
            .inner_size(500.0, 650.0)
            .build()
            .map_err(|e| e.to_string())?;
            Some(window) // Return the created window
        }
        #[cfg(feature = "open")]
        {
            open::that(&oauth_url_str)
                .map_err(|e| format!("Failed to open URL in browser: {}", e))?;
            None
        }
    };

    let auth_info =
        tauri::async_runtime::spawn(
            async move { builder.launch().await.map_err(|e| e.to_string()) },
        )
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;

    if let Some(window) = webview_window_to_close {
        if auth_info.access_token.is_some() {
            let _ = window.close();
        }
    }

    Ok(auth_info)
}
