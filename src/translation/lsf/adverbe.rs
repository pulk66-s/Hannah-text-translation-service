use crate::logger::log;
use crate::AST::adverbe::Adv;

pub fn translate_adverbe(adverbe: &Adv) -> String {
    adverbe.get()
}