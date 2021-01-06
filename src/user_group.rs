// TODO think of other properties that are particular to groups.
/// A clan.
/// This is a group of multiple users.
pub struct UserGroup {
    /// The id of this clan.
    pub id: i32,
    /// The users composing this clan.
    pub users: Vec<i32>,
}

/// The settings of user groups.
pub struct UserGroupSettings {
    /// The maximum number of users in a group.
    pub maximum_users: i32,
}
