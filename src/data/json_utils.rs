use std::fs;
use serde_json;

use crate::models::Profile;
use anyhow::{Result, Context};

const PROFILE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/data/myProfile.json");

pub fn load_profile () -> Result<Profile> {
    let file = fs::read_to_string(PROFILE_FILE).with_context(|| format!("Failed to read profile file: {}", PROFILE_FILE))?;
    let profile: Profile = serde_json::from_str(&file).with_context(|| format!("Failed to parse profile JSON: {}", file))?;
    Ok(profile)
}