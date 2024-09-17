use crate::logger::log;
use crate::{
    AST::phrase::{
        Phrase,
        simple::SimplePhrase,
        complex::{
            ComplexPhrase,
            juxtaposition::Juxtaposition,
            coordination::Coordination,
            subordination::Subordination
        },
        proposition::Proposition
    },
    translation::lsf::{
        nominal_group::translate_nominal_group,
        verb::translate_group_verbal,
        prepositional_group::translate_group_prepositional
    }
};

pub fn translate_phrase(phrase: &Phrase) -> String {
    let left = phrase.phr().left();
    let right = phrase.phr().right();
    let mut res = String::new();
    
    if let Some(left) = left {
        res += &translate_Simple_phrase(&left);
    } else if let Some(right) = right {
        res += &translate_complex_phrase(&right);
    }
    log::new().syntax(&format!("translate_phrase: {}", res));
    res
}

pub fn translate_proposition(proposition: &Proposition) -> String {
    let mut res = String::new();

    if let Some (expr) = proposition.gn().expr() {
        log::new().syntax(&format!("translate_nominal"));
        res += &translate_nominal_group(&expr);
    }
    if let Some (expr) = proposition.verb().expr() {
        log::new().syntax(&format!("translate_group_verbal"));
        res += &translate_group_verbal(&expr);
    }
    if let Some (expr) = proposition.complement().expr() {
        log::new().syntax(&format!("translate_group_prepositional"));
        res += &translate_group_prepositional(&expr);
    }
    log::new().syntax(&format!("translate_proposition: {}", res));
    res
}

pub fn translate_Simple_phrase(phrase: &SimplePhrase) -> String {
    translate_proposition(phrase.proposition())
}

pub fn translate_juxtaposition(prop: &Juxtaposition) -> String {
    let mut res = String::new();

    res += &translate_proposition(prop.proposition().left());
    res += " ";
    res
}

pub fn translate_coordination(prop: &Coordination) -> String {
    let mut res = String::new();

    res += &translate_proposition(prop.proposition().left());
    res += " ";
    res
}

pub fn translate_subordination(prop: &Subordination) -> String {
    let mut res = String::new();

    res += &translate_proposition(prop.proposition().right());
    res += " ";
    res
}

pub fn translate_complex_phrase(phrase: &ComplexPhrase) -> String {
    let mut res = String::new();

    for expr in &phrase.data().items {
        if let Some(expr) = expr.left() {
            res += &translate_juxtaposition(&expr);
            res += " ";
        } else if let Some(expr) = expr.right() {
            if let Some (expr) = expr.left() {
                res += &translate_coordination(&expr);
                res += " ";
            } else if let Some (expr) = expr.right() {
                res += &translate_subordination(&expr);
                res += " ";
            }
        }
    }
    res
}
