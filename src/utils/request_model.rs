#[derive(serde::Deserialize)]
pub struct CreateTodoRequest {
    pub text: String,
}