use crate::models::User;
use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "soulbook_token";

/// Global trigger: set to true to open the "新建文档" modal on the Docs page.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateDocTrigger(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub token: Option<String>,
    pub user: Option<User>,
}

impl AuthState {
    pub fn init() -> Self {
        let token = LocalStorage::get::<String>(TOKEN_KEY)
            .ok()
            .or_else(|| LocalStorage::get::<String>("souldoc_token").ok())
            .or_else(|| LocalStorage::get::<String>("jwt_token").ok())
            .or_else(|| LocalStorage::get::<String>("auth_token").ok())
            .or_else(|| LocalStorage::get::<String>("token").ok());
        Self { token, user: None }
    }

    pub fn login(&mut self, token: String, user: User) {
        LocalStorage::set(TOKEN_KEY, &token).ok();
        LocalStorage::set("souldoc_token", &token).ok();
        self.token = Some(token);
        self.user = Some(user);
    }

    pub fn logout(&mut self) {
        LocalStorage::delete(TOKEN_KEY);
        LocalStorage::delete("souldoc_token");
        self.token = None;
        self.user = None;
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
}
