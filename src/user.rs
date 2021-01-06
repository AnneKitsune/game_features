/// Base struct for the user of a game.
#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct User<T> {
    /// The numerical id of the user.
    pub id: i32,
    /// The name of the user.
    pub name: String,
    /// Extra data associated with the user.
    ///
    /// Note: this is not where you want to store your hashed pashwords.
    pub data: T,
}

/// The list of all known users.
pub type UserRepository<T> = Vec<User<T>>;
