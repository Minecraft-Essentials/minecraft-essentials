#![deny(clippy::all)]

use napi::{Error, Result, bindgen_prelude::*};
use napi_derive::napi;

use core::EXPERIMENTAL_MESSAGE;
use core::HTTP::Client;
use core::auth::AuthInfo as CoreAuthInfo;
use core::launch::JavaJRE;

use core::auth::{
    bearer_token,
    microsoft::{SCOPE, authenticate_device, device_authentication_code, ouath, ouath_token},
    xbox::{XblOutput, XtsOutput, xbl, xsts},
};

#[napi]
#[derive(Debug, PartialEq, Eq)]
pub enum AuthType {
    Oauth,
    DeviceCode,
}

#[napi(object)]
pub struct AuthInfo {
    pub device_code: Option<String>,
    pub ouath_url: Option<String>,
}

#[napi(object)]
pub struct AuthData {
    pub access_token: Option<String>,
    pub uuid: Option<String>,
    pub expires_in: i64,
    pub xts_token: Option<String>,
}

impl From<CoreAuthInfo> for AuthData {
    fn from(c: CoreAuthInfo) -> Self {
        AuthData {
            access_token: c.access_token,
            uuid: c.uuid,
            expires_in: c.expires_in as i64, // safe conversion
            xts_token: c.xts_token,
        }
    }
}

#[napi]
pub struct AuthBuilder {
    auth_type: AuthType,
    client_id: String,
    port: u16,
    client_secret: String,
    bedrockrel: bool,
}

#[napi]
impl AuthBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            auth_type: AuthType::Oauth,
            client_id: "".to_string(),
            port: 8000,
            client_secret: "".to_string(),
            bedrockrel: false,
        }
    }

    #[napi]
    pub fn set_type(&mut self, auth_type: AuthType) -> &Self {
        self.auth_type = auth_type;
        self
    }

    #[napi]
    pub fn set_client_id(&mut self, client_id: String) -> &Self {
        self.client_id = client_id;
        self
    }

    #[napi]
    pub fn set_port(&mut self, port: u16) -> &Self {
        self.port = port;
        self
    }

    #[napi]
    pub fn set_client_secret(&mut self, client_secret: String) -> &Self {
        self.client_secret = client_secret;
        self
    }

    #[napi]
    pub fn set_bedrockrel(&mut self, bedrockrel: bool) -> &Self {
        self.bedrockrel = bedrockrel;
        self
    }
    #[napi]
    pub fn get_self(&self, env: Env) -> Result<Object> {
        let mut obj = env.create_object()?;

        obj.set("type", self.auth_type as i32)?;
        obj.set("client_id", self.client_id.clone())?;
        obj.set("port", self.port)?;
        obj.set("client_secret", self.client_secret.clone())?;
        obj.set("bedrockrel", self.bedrockrel)?;

        Ok(obj)
    }

    #[napi]
    pub async fn get_info(&self) -> Result<AuthInfo> {
        let client = Client::new();

        match self.auth_type {
            AuthType::DeviceCode => {
                let code = device_authentication_code(client, &self.client_id)
                    .await
                    .map_err(|e| Error::from_reason(format!("Device code error: {}", e)))?;

                Ok(AuthInfo {
                    device_code: Some(code.user_code),
                    ouath_url: None,
                })
            }
            AuthType::Oauth => {
                let url = format!(
                    "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize/?client_id={}&response_type=code&redirect_uri=http://localhost:{}&response_mode=query&scope={}&state=12345",
                    self.client_id, self.port, SCOPE
                );

                Ok(AuthInfo {
                    device_code: None,
                    ouath_url: Some(url),
                })
            }
        }
    }

    #[napi]
    pub async fn launch(&self) -> Result<AuthData> {
        let client = Client::new();

        // Helper function to handle the common response part
        async fn process_tokens(
            client: &Client,
            xbl_resp: &XblOutput,
            xts_resp: &XtsOutput,
            bedrockrel: bool,
        ) -> Result<AuthData> {
            if bedrockrel {
                Ok(AuthData {
                    access_token: None,
                    uuid: None,
                    expires_in: 0,
                    xts_token: Some(xts_resp.token.clone()),
                })
            } else {
                let custom_data = bearer_token(
                    client.clone(),
                    &xbl_resp.display_claims.xui[0].uhs,
                    &xts_resp.token,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Bearer token failed: {}", e)))?;

                Ok(custom_data.into())
            }
        }

        match self.auth_type {
            AuthType::Oauth => {
                let server = ouath(self.port)
                    .map_err(|e| Error::from_reason(format!("OAuth server error: {}", e)))?
                    .await
                    .map_err(|e| Error::from_reason(format!("OAuth response error: {}", e)))?;

                let code = server.code.ok_or_else(|| {
                    Error::from_reason("OAuth code missing from response".to_string())
                })?;

                let token = ouath_token(
                    client.clone(),
                    &code,
                    &self.client_id,
                    self.port,
                    &self.client_secret,
                )
                .await
                .map_err(|e| Error::from_reason(format!("Token exchange failed: {}", e)))?;

                let xbl_resp = xbl(client.clone(), &token.access_token)
                    .await
                    .map_err(|e| Error::from_reason(format!("XBL auth failed: {}", e)))?;

                let xts_resp = xsts(client.clone(), &xbl_resp.token, self.bedrockrel)
                    .await
                    .map_err(|e| Error::from_reason(format!("XSTS auth failed: {}", e)))?;

                process_tokens(&client, &xbl_resp, &xts_resp, self.bedrockrel).await
            }

            AuthType::DeviceCode => {
                println!("{} \nStatus: WIP (Work In Progress)", EXPERIMENTAL_MESSAGE);

                let code = device_authentication_code(client.clone(), &self.client_id)
                    .await
                    .map_err(|e| Error::from_reason(format!("Device code error: {}", e)))?;

                let code_token =
                    authenticate_device(client.clone(), &code.device_code, &self.client_id)
                        .await
                        .map_err(|e| Error::from_reason(format!("Auth device failed: {}", e)))?;

                let xbl_resp = xbl(client.clone(), &code_token.token)
                    .await
                    .map_err(|e| Error::from_reason(format!("XBL failed: {}", e)))?;

                let xts_resp = xsts(client.clone(), &xbl_resp.token, self.bedrockrel)
                    .await
                    .map_err(|e| Error::from_reason(format!("XSTS failed: {}", e)))?;

                process_tokens(&client, &xbl_resp, &xts_resp, self.bedrockrel).await
            }
        }
    }
}

#[napi]
pub struct LaunchBuilder {
    args: Vec<String>,
    java_path: Option<String>,
    client: Option<String>,
    mods: Option<Vec<String>>,
}

#[napi(object)]
pub struct LauncherDirs {
    pub game_dir: String,
    pub assets_dir: String,
    pub libraries_dir: String,
    pub natives_dir: String,
    pub java_dir: String,
}

#[napi]
pub enum JsJavaJRE {
    Adoptium,
    Zulu,
    GraalVM,
}

impl From<JsJavaJRE> for JavaJRE {
    fn from(js_java_jre: JsJavaJRE) -> Self {
        match js_java_jre {
            JsJavaJRE::Adoptium => JavaJRE::Adoptium,
            JsJavaJRE::Zulu => JavaJRE::Zulu,
            JsJavaJRE::GraalVM => JavaJRE::GraalVM,
        }
    }
}

impl From<JavaJRE> for JsJavaJRE {
    fn from(java_jre: JavaJRE) -> Self {
        match java_jre {
            JavaJRE::Adoptium => JsJavaJRE::Adoptium,
            JavaJRE::Zulu => JsJavaJRE::Zulu,
            JavaJRE::GraalVM => JsJavaJRE::GraalVM,
        }
    }
}

#[napi]
impl LaunchBuilder {
    #[napi(constructor)]
    pub fn new() -> Self {
        LaunchBuilder {
            args: vec![],
            java_path: None,
            client: None,
            mods: None,
        }
    }

    #[napi]
    pub fn set_args(&mut self, args: Vec<String>) -> &Self {
        self.args = args;
        self
    }

    #[napi]
    pub fn set_java(&mut self, path: Option<String>) -> &Self {
        self.java_path = path;
        self
    }

    #[napi]
    pub fn set_client(&mut self, client: Option<String>) -> &Self {
        self.client = client;
        self
    }

    #[napi]
    pub fn set_mods(&mut self, mods: Option<Vec<String>>) -> &Self {
        self.mods = mods;
        self
    }

    #[napi]
    pub fn get_self(&self, env: Env) -> Result<Object> {
        let mut obj = env.create_object()?;

        // Return all the fields as plain object
        obj.set("args", self.args.clone())?;
        obj.set("java_path", self.java_path.clone())?;
        obj.set("client", self.client.clone())?;
        obj.set("mods", self.mods.clone())?;

        Ok(obj)
    }

    #[napi]
    pub fn launch(&self, jre: Option<JsJavaJRE>, dirs: Option<LauncherDirs>) {
        let mut args = self.args.clone();
        if cfg!(target_os = "macos") {
            args.push("-XstartOnFirstThread".to_string());
        }
    }
}
