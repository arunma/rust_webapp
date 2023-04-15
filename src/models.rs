use crate::schema::cookies;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Cookie {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

#[derive(Serialize, Insertable, Deserialize, Debug)]
#[table_name = "cookies"]
pub struct NewCookie {
    pub name: String,
    pub image_path: String,
}
