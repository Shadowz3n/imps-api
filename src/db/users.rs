use actix::prelude::*;
use diesel::prelude::*;
use libreauth::pass::HashBuilder;

use super::DbExecutor;
use crate::app::users::{LoginUser, RegisterUser, UpdateUserOuter, UserResponse};
use crate::models::{NewUser, User, UserChange};
use crate::prelude::*;
use crate::utils::{HASHER, PWD_SCHEME_VERSION};

impl Message for RegisterUser {
    type Result = Result<UserResponse>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let new_user = NewUser {
            email: msg.email.clone(),
            pass: HASHER.hash(&msg.pass)?,
        };

        let conn = &self.0.get()?;

        match diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(conn)
        {
            Ok(user) => Ok(user.into()),
            Err(e) => Err(e.into()),
        }
    }
}

impl Message for LoginUser {
    type Result = Result<UserResponse>;
}

impl Handler<LoginUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: LoginUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let provided_pass_raw = &msg.pass;

        let conn = &self.0.get()?;

        let stored_user: User = users.filter(email.eq(msg.email)).first(conn)?;
        let checker = HashBuilder::from_phc(&stored_user.pass)?;

        if checker.is_valid(provided_pass_raw) {
            if checker.needs_update(PWD_SCHEME_VERSION) {
                let new_pass = HASHER.hash(provided_pass_raw)?;
                return match diesel::update(users.find(stored_user.id))
                    .set(pass.eq(new_pass))
                    .get_result::<User>(conn)
                {
                    Ok(user) => Ok(user.into()),
                    Err(e) => Err(e.into()),
                };
            }
            Ok(stored_user.into())
        } else {
            Err(Error::Unauthorized(json!({
                "error": "Wrong password",
            })))
        }
    }
}

impl Message for UpdateUserOuter {
    type Result = Result<UserResponse>;
}

impl Handler<UpdateUserOuter> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: UpdateUserOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let auth = msg.auth;
        let update_user = msg.update_user;

        let conn = &self.0.get()?;

        let updated_pass = match update_user.pass {
            Some(updated_pass) => Some(HASHER.hash(&updated_pass)?),
            None => None,
        };

        let updated_user = UserChange {
            email: update_user.email,
            pass: updated_pass,
        };

        match diesel::update(users.find(auth.user.id))
            .set(&updated_user)
            .get_result::<User>(conn)
        {
            Ok(user) => Ok(user.into()),
            Err(e) => Err(e.into()),
        }
    }
}