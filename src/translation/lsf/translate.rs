use crate::logger::log;
use crate::{
    AST::text::Text,
    translation::lsf::text::translate_text
};

///
/// Brief: Translates the AST into LSF
/// LSF = Langue des Signes Française (French Sign Language)
/// 
pub fn lsf_translation(text: Text) -> String {
    translate_text(text).trim().to_string()
}