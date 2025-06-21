use diesel::{
    prelude::{Insertable, Queryable},
    ExpressionMethods, QueryDsl, RunQueryDsl, Selectable, SelectableHelper,
};
use uuid::Uuid;

use crate::store::Store;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: String,
    username: String,
    password: String,
}

impl Store {
    pub fn sign_up(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<String, diesel::result::Error> {
        use crate::schema::user;
        let id = Uuid::new_v4();
        let user = User {
            username: input_username,
            password: input_password,
            id: id.to_string(),
        };
        diesel::insert_into(user::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;

        Ok(id.to_string())
    }

    pub fn sign_in(
        &mut self,
        input_username: String,
        input_password: String,
    ) -> Result<bool, diesel::result::Error> {
        use crate::schema::user::dsl::*;
        let user_result = user
            .filter(username.eq(input_username))
            .select(User::as_select())
            .first(&mut self.conn)?;
        if user_result.password != input_password {
            return Ok(false);
        }
        Ok(true)
    }
}
