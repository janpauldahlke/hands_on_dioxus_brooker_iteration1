use dioxus::prelude::*;
use crate::server::profile::load_profile_server;

const PROFILE_CSS: Asset = asset!("/assets/styling/profile.css");

#[component]
pub fn Profile() -> Element {
    let profile = use_server_future(move || load_profile_server())?;
    
    match profile() {
        Some(Ok(profile_data)) => {
            rsx! {
                document::Link { rel: "stylesheet", href: PROFILE_CSS }
                div { id: "profile", class: "container",
                    h1 { "Welcome {profile_data.get_profile_name()" }
                    ul {
                        li { "Name: {profile_data.get_name()}" }
                        li { "Email: {profile_data.get_email()}" }
                        li { "Password: {profile_data.get_password()}" }
                        li { "Created At: {profile_data.get_created_at()}" }
                        li { "Updated At: {profile_data.get_updated_at()}" }
                    }
                }
            }
        }
        Some(Err(e)) => {
            rsx! {
                div { "Error loading profile: {e}" }
            }
        }
        None => {
            rsx! {
                div { "Loading profile..." }
            }
        }
    }
}