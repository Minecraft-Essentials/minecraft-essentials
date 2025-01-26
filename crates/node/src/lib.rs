#![deny(clippy::all)]

use minecraft_essentials::Oauth as MinecraftOauth;
use minecraft_essentials::AuthData;
use napi::bindgen_prelude;

#[macro_use]
extern crate napi_derive;



#[napi]
pub struct Oauth {
    oauth: minecraft_essentials::Oauth,
}


use napi::{CallContext, Env, JsObject, JsString, JsUnknown, Result};
use napi_derive::module_exports;

pub struct AuthDataWrapper {
    inner: minecraft_essentials::AuthData,
}

impl AuthDataWrapper {
    pub fn new(inner: minecraft_essentials::AuthData) -> Self {
        Self { inner }
    }
}

#[napi]
impl AuthDataWrapper {
    #[napi(getter)]
    pub fn get_access_token(&self, ctx: CallContext) -> Result<JsString> {
        let env = ctx.env;
        let access_token = self.inner.access_token; // Access the access_token field directly
        env.create_string(&access_token)
    }

    // Implement other getters or methods as needed to expose the necessary data
}



#[napi]
impl Oauth {
 #[napi(constructor)]
 pub async fn new(client_id: String, port: u16) -> Self {
    let oauth = minecraft_essentials::Oauth::new(client_id.as_str(), Some(port));
    Self { oauth }
 }

 #[napi]
 pub fn url(&self) -> &str {
    &self.oauth.url()
 }

 #[napi]
 pub async fn launch(&self, client_secret: String, bedrockRel: bool) -> Result<AuthData, Box<dyn std::error::Error>> {
     let oauth_info = self.oauth.launch(bedrockRel, &client_secret).await.map_err(|e| {
         // Convert the error `e` into the desired error type here
         // For example, if `e` is a custom error type, you might do something like this:
         Err(Some(format!("{}", e)))
     })?;
     Ok(oauth_info)
 }

}

#[napi]
pub struct DeviceCode {
  url: String,
}

#[napi]
impl DeviceCode {
  #[napi(constructor)]
  pub fn new(client_id: String, port: u16 ) -> Self {
    println!("\x1b[33mNOTICE: You are using and Experiemntal Feature.\x1b[0m");

    let url = format!("fjnbdsjkhfbsdgfbejsd");

    Self { url }
  }

  #[napi]
  pub fn prelaunch(&self) -> String {
    self.url.clone()
  }

  #[napi]
  pub fn launch() -> String {
    "Device launch called".to_string()
  }
}

