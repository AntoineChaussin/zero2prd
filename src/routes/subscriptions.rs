use axum::extract::rejection::FormRejection;
use axum::extract::Form;
use http::StatusCode;

pub async fn subscription(
    form: Result<Form<FormData>, FormRejection>,
) -> Result<StatusCode, StatusCode> {
    match form {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
