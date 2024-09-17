use crate::logger::log;
use crate::{
    AST::text::Text,
    translation::lsf::phrase::translate_phrase
};

pub fn translate_text(text: Text) -> String {
    text.phrases.items.iter().map(|phrase| {
        translate_phrase(phrase)
    }).collect::<Vec<String>>().join(" ")
}