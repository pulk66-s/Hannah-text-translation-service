use crate::logger::log;
use crate::AST::adjectives::adjectival_group::AdjectivalGroup;

pub fn translate_adjectival_group(group: &AdjectivalGroup) -> String {
    let mut res = String::new();
    let list = &group.adj().items;

    for l in list {
        let right = &l.right();

        res += &right.get();
        res += " ";
    }
    res.trim().to_string()
}
