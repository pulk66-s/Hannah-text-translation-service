use crate::logger::log;
use crate::AST::subject::nominal_group::NominalGroup;

use super::pronouns::translate_pronoun;

pub fn translate_nominal_group(group: &NominalGroup) -> String {
    let mut res = String::new();

    match group.noun().left() {
        Some(ref pronoun) => {
            res += &translate_pronoun(pronoun);
            res += " ";
        },
        None => match group.noun().right() {
            Some(ref noun) => {
                res += &noun.get_word();
                res += " ";
            },
            None => {
                res += "Unknown";
                res += " ";
            },
        }
    };
    if let Some(adj) = group.adjectives().expr() {
        let adjs = &adj.adj().items;

        for adj in adjs {
            res += &adj.right().get();
            res += " ";
        }
    }
    log::new().syntax(&format!("translate_nominal_group: {}", res));
    res
}