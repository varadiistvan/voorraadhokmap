struct Crate {}

struct CrateType {}

struct Item {
    name: String,
    aliases: Vec<String>,
}

struct Map {}

struct MapAuthorization {}

struct Stack {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrateOrientation {
    ShortX,
    ShortY,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrincipalType {
    User,
    Group,
}
