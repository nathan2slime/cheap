use sea_orm::DbErr;

#[derive(Debug)]
pub enum SignUp {
    EmailInUse,
    InternalSignUp(DbErr),
}

#[derive(Debug)]
pub enum SignIn {
    InvalidCredentials,
    InternalSignIn(DbErr),
}
