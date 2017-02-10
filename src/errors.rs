error_chain! {
    links {
    }
    foreign_links {
        Io(::std::io::Error) #[doc = "Link to a `std::error::Error` type."];
        EnvVar(::std::env::VarError) #[doc = "Link to a `std::env::VarError` type."];
        Hyper(::hyper::Error) #[doc = "Link to a `hyper::Error` type."];
        Log(::log::SetLoggerError) #[doc = "Link to a `log::SetLoggerError` type."];
        Body(::bodyparser::BodyError);
        //Poison(::std::sync::PoisonError);
    }
    errors {
        #[doc = "A custom error kind."]
        Custom
    }
}
use iron;
use std;

impl std::convert::Into<iron::IronError> for self::Error{
    fn into(self) -> iron::IronError {
        iron::IronError::new(self, iron::status::InternalServerError)
    }
}