use dioxus::prelude::*;
use crate::data::json_utils::load_profile;

use crate::models::Profile;
use anyhow::Result;


#[post("/api/profile/load")]
pub async fn load_profile_server() -> Result<Profile, ServerFnError> {
    load_profile()
        .map_err(|e| ServerFnError::new(e.to_string()))
}