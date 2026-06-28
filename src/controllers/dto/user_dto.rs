use serde::Deserialize;

use crate::domain::models::{
    user::{UserCommand, UserInput},
    user_status::UserRole,
};

#[derive(Debug, Deserialize)]
pub struct SignUp {
    name: String,
    email: String,
    password: String,
    phone_number: String,
    role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

impl SignUp {
    pub fn into_domain(self) -> UserCommand {
        UserCommand::new(
            self.name,
            self.email,
            self.password,
            self.phone_number,
            self.role,
        )
    }
}
#[derive(Debug, Deserialize)]
pub struct UpdateUserStatusDto {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<UserRole>,
}

impl UpdateUserDto {
    pub fn into_input(self) -> UserInput {
        UserInput {
            name: self.name,
            email: self.email,
            phone_number: self.phone_number,
            role: self.role,
        }
    }
}
