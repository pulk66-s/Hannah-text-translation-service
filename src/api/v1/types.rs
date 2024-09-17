use serde::Deserialize;

#[derive(Deserialize)]
pub struct TranslateForm {
    pub text: String,
}
