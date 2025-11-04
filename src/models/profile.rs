use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    profile_id: i32,
    profile_name: String,
    name: String,
    email: String,
    password: String,
    created_at: String, // epoch
    updated_at: String, //epoch
    img: Option<String>
}

impl Profile {
    pub fn new(profile_id: i32, profile_name: String, name: String, email: String, password: String, created_at: String, updated_at: String, img: Option<String>) -> Self {
        Self { profile_id, profile_name, name, email, password, created_at, updated_at, img }
    }
}

impl Profile {
    pub fn get_profile_id(&self) -> &i32 {
        &self.profile_id
    }
    pub fn get_profile_name(&self) -> &str {
        &self.profile_name
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn get_created_at(&self) -> &str {
        &self.created_at
    }
    pub fn get_updated_at(&self) -> &str {
        &self.updated_at
    }
    pub fn get_img(&self) -> &Option<String> {
        &self.img
    }


    // pub fn create_profile(profile_name: String, name: String, email: String, password: String) -> Self {
    //     Self { profile_id: 0, profile_name, name, email, password, created_at: String::new(), updated_at: String::new(), img: None }
    // }
    // pub fn update_profile(profile_id: i32, profile_name: String, name: String, email: String, password: String) -> Self {
    //     Self { profile_id, profile_name, name, email, password, created_at: String::new(), updated_at: String::new(), img: None }
    // }
    // pub fn delete_profile(profile_id: i32) -> Self {
    //     Self { profile_id, profile_name: String::new(), name: String::new(), email: String::new(), password: String::new(), created_at: String::new(), updated_at: String::new(), img: None }
    // }
    // pub fn get_profile(profile_id: i32) -> Self {
    //     Self { profile_id, profile_name: String::new(), name: String::new(), email: String::new(), password: String::new(), created_at: String::new(), updated_at: String::new(), img: None }
    // }
}

