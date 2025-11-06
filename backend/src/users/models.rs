use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

#[derive(Debug, Deserialize)]
pub struct UpdateEmailPayload {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePasswordPayload {
    pub password: String,
}

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("id", 3)?;
        s.serialize_field("id", &self.id.to_string())?;
        s.serialize_field("email", &self.email)?;
        s.serialize_field("first_name", &self.first_name)?;
        s.serialize_field("last_name", &self.last_name)?;
        s.end()
    }
}
