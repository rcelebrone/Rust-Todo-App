use rocket::request::FromParam;
use uuid::Uuid;

// Defines a new type that wraps the Uuid
pub struct UuidParam(pub Uuid);

// Implements FromParam for the new type
impl<'a> FromParam<'a> for UuidParam {
    type Error = uuid::Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(UuidParam(Uuid::parse_str(param)?))
    }
}