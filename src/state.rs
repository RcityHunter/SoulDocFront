use crate::models::User;
use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "soulbook_token";
const WORKSPACE_KEY: &str = "soulbook_active_workspace";

/// Global trigger: set to true to open the "新建文档" modal on the Docs page.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateDocTrigger(pub bool);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceKind {
    Personal,
    Team,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamWorkspace {
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceState {
    pub active_kind: WorkspaceKind,
    pub active_team_id: Option<String>,
    pub teams: Vec<TeamWorkspace>,
}

impl WorkspaceState {
    pub fn init() -> Self {
        let active = LocalStorage::get::<String>(WORKSPACE_KEY).ok();
        workspace_state_from_saved(active, Vec::new())
    }

    pub fn select_personal(&mut self) {
        self.active_kind = WorkspaceKind::Personal;
        self.active_team_id = None;
        LocalStorage::set(WORKSPACE_KEY, "personal").ok();
    }

    pub fn select_team(&mut self, team_id: &str) {
        if self.teams.iter().any(|team| team.id == team_id) {
            self.active_kind = WorkspaceKind::Team;
            self.active_team_id = Some(team_id.to_string());
            LocalStorage::set(WORKSPACE_KEY, format!("team:{team_id}")).ok();
        } else {
            self.select_personal();
        }
    }

    pub fn active_team(&self) -> Option<&TeamWorkspace> {
        let active_id = self.active_team_id.as_deref()?;
        self.teams.iter().find(|team| team.id == active_id)
    }
}

fn workspace_state_from_saved(
    saved: Option<String>,
    teams: Vec<TeamWorkspace>,
) -> WorkspaceState {
    if let Some(team_id) = saved
        .as_deref()
        .and_then(|value| value.strip_prefix("team:"))
        .filter(|team_id| teams.iter().any(|team| team.id == *team_id))
    {
        WorkspaceState {
            active_kind: WorkspaceKind::Team,
            active_team_id: Some(team_id.to_string()),
            teams,
        }
    } else {
        WorkspaceState {
            active_kind: WorkspaceKind::Personal,
            active_team_id: None,
            teams,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_defaults_to_personal_without_saved_team() {
        let state = workspace_state_from_saved(None, Vec::new());

        assert_eq!(state.active_kind, WorkspaceKind::Personal);
        assert_eq!(state.active_team_id, None);
    }

    #[test]
    fn workspace_falls_back_to_personal_when_saved_team_is_missing() {
        let state = workspace_state_from_saved(Some("team:missing".to_string()), Vec::new());

        assert_eq!(state.active_kind, WorkspaceKind::Personal);
        assert_eq!(state.active_team_id, None);
    }

    #[test]
    fn workspace_restores_saved_team_when_team_exists() {
        let state = workspace_state_from_saved(
            Some("team:t1".to_string()),
            vec![TeamWorkspace {
                id: "t1".to_string(),
                name: "研发团队".to_string(),
                role: "成员".to_string(),
            }],
        );

        assert_eq!(state.active_kind, WorkspaceKind::Team);
        assert_eq!(state.active_team_id.as_deref(), Some("t1"));
        assert_eq!(state.active_team().map(|team| team.name.as_str()), Some("研发团队"));
    }
}
