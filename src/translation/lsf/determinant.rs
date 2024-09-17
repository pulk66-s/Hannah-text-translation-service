use crate::logger::log;
use crate::AST::determinants::Determinant;

pub fn translate_determinant(determinant: &Determinant) -> String {
    determinant.get_word()
}