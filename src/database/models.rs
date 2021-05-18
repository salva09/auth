use crate::database::schema::users;

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str
}
