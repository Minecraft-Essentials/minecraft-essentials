use std::{env, fs, path::PathBuf};

use crate::{errors::LaunchErrors, trait_alias::AsyncSendSync, HTTP::Client};
use super::download_files;

/// Java Runtime Environment (JRE) for Minecraft.
#[derive(Debug, Clone)]
pub enum JRE {
    Adoptium,
    Zulu,
    GraalVM,
}

struct ArchUrl {
    arch: Option<&'static str>,
    os: Option<&'static str>,
    url: String,
}

fn java_url(jre: JRE, version: &str) -> Option<String> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    match jre {
        JRE::Adoptium => {
            arch_support(&["x86_64", "x86", "aarch64", "arm"]).ok()?;

            let urls = vec![
                ArchUrl {
                    arch: Some("x86_64"),
                    os: None,
                    url: format!(
                        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/x64/jre/hotspot/normal/eclipse",
                        version, os
                    ),
                },
                ArchUrl {
                    arch: None,
                    os: None,
                    url: format!(
                        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
                        version, os, arch
                    ),
                },
            ];
            arch_url(urls)
        }

        JRE::Zulu => todo!(),

        JRE::GraalVM => {
            arch_support(&["x86_64", "x86", "aarch64"]).ok()?;

            let urls = vec![
                ArchUrl {
                    arch: Some("x86"),
                    os: Some("windows"),
                    url: format!(
                        "https://download.oracle.com/graalvm/{}/latest/graalvm-jdk-{}_windows-x64_bin.zip",
                        version, version
                    ),
                },
                ArchUrl {
                    arch: None,
                    os: None,
                    url: format!(
                        "https://download.oracle.com/graalvm/{}/latest/graalvm-jdk-{}_{}-{}_bin.tar.gz",
                        version, version, os, arch
                    ),
                },
            ];
            arch_url(urls)
        }
    }
}

pub fn get_java(
    client: Client,
    dir: &PathBuf,
    version: &str,
    jre: JRE,
    user_agent: &str,
) -> impl AsyncSendSync<Result<(), LaunchErrors>> {
    let output_path = {
        let mut path = dir.clone();
        if cfg!(target_os = "windows") {
            path.push("jre.zip");
        } else {
            path.push("jre.tar.gz");
        }
        path
    };

    let user_agent = user_agent.to_owned();
    let version = version.to_owned(); 
    let jre = jre.clone();

    async move {
        let url = java_url(jre, &version).ok_or_else(|| {
            LaunchErrors::Requirements("Unsupported platform or JRE URL not found".to_string())
        })?;

        download_jre(client, url, output_path, &user_agent).await
    }
}


async fn download_jre(
    client: Client,
    url: String,
    path: PathBuf,
    user_agent: &str,
) -> Result<(), LaunchErrors> {
    download_files(client.clone(), user_agent, &path, url).await.map_err(|e| {
        LaunchErrors::Requirements(format!("Failed to download JRE due to: {}", e))
    })?;

    if path.exists() && path.is_file() {
        fs::remove_file(&path).map_err(|e| {
            LaunchErrors::Requirements(format!("Failed to clean up file: {}", e))
        })?;
    }

    Ok(())
}

fn arch_url(candidates: Vec<ArchUrl>) -> Option<String> {
    let arch = env::consts::ARCH;
    let os = env::consts::OS;

    candidates.into_iter().find_map(|entry| match (entry.arch, entry.os) {
        (Some(a), Some(o)) if a == arch && o == os => Some(entry.url),
        (Some(a), None) if a == arch => Some(entry.url),
        (None, Some(o)) if o == os => Some(entry.url),
        (None, None) => Some(entry.url),
        _ => None,
    })
}

fn arch_support(supported: &[&str]) -> Result<(), LaunchErrors> {
    if supported.contains(&env::consts::ARCH) {
        Ok(())
    } else {
        Err(LaunchErrors::UnsupportedArchitecture(
            env::consts::ARCH.to_string(),
        ))
    }
}
