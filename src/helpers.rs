use diesel::result::Error as DieselError;
use rocket::http::Status;

pub fn diesel_error_to_status(error: DieselError) -> Status {
    match error {
        DieselError::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
