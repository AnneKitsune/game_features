/// Clan
pub struct UserGroup {
    pub id: i32,
    pub users: Vec<i32>,
}

pub struct UserGroupSettings {
    pub maximum_users: i32,
    pub friendly_fire: bool,
}
