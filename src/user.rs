#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub struct User<T> {
    pub id: i32,
    pub name: String,
    pub data: T,
}

pub type UserRepository<T> = Vec<User<T>>;

