pub mod defined_articles;
pub mod demonstrative_adjectives;
pub mod undefined_articles;
pub mod numerals_adjectives;
pub mod partial_articles;
pub mod possessive_adjectives;
pub mod undefined_adjectives;

use defined_articles::DefinedArticles;
use demonstrative_adjectives::DemonstrativeAdjectives;
use undefined_articles::UndefinedArticles;
use numerals_adjectives::NumeralsAdjectives;
use partial_articles::PartialArticles;
use possessive_adjectives::PossessiveAdjectives;
use undefined_adjectives::UndefinedAdjectives;

use std::fmt;
use crate::logger::log;
use crate::AST::{
    expr::Expr,
    utils::skipable::Skipable,
};
use crate::context::Context;

use super::utils::or::Or;
use super::word::Word;

pub struct Determinant<'a> {
    context: &'a Context,
    // determinant: Or<'a,
    //     DefinedArticles<'a>,
    //     Or<'a,
    //         DemonstrativeAdjectives<'a>,
    //         Or<'a,
    //             UndefinedAdjectives<'a>,
    //             Or<'a,
    //                 UndefinedArticles<'a>,
    //                 Or<'a,
    //                     NumeralsAdjectives<'a>,
    //                     Or<'a,
    //                         PartialArticles<'a>,
    //                         PossessiveAdjectives<'a>
    //                     >
    //                 >
    //             >
    //         >
    //     >
    // >
    determinant: Word<'a>
}

impl<'a> Expr<'a> for Determinant<'a> {
    fn new(context: &'a Context) -> Self {
        Determinant {
            context: &context,
            // determinant: Or::new(&context)
            determinant: Word::new(&context)
        }
    }

    // fn parse(&mut self, input: String) -> (String, bool) {
    //     log::new().syntax(&format!("Determinant::parse({})", input));
    //     let old_input = input.clone();
    //     let (input, _) = Skipable::new(self.context).parse(input.clone());
    //     let (input, success) = self.determinant.parse(input.clone());

    //     if !success {
    //         log::new().syntax(&format!("Determinant::parse({}) 1 -> ({}, {})", old_input, old_input, false));
    //         return (old_input, false)
    //     }
    //     log::new().syntax(&format!("Determinant::parse({}) 2 -> ({}, {})", old_input, input, success));
    //     (input, success)
    // }

    fn parse(&mut self, input: String) -> (String, bool) {
        log::new().syntax(&format!("Determinant::parse({})", input));
        let old_input = input.clone();
        let (input, _) = Skipable::new(self.context).parse(input.clone());
        let (input, success) = self.determinant.parse(input.clone());
        let key = self.determinant.get();

        if !success {
            log::new().syntax(&format!("Determinant::parse({}) 1 -> ({}, {})", old_input, old_input, false));
            return (old_input, false)
        }
        return match self.context.get_determinant(&key) {
            Some(determinant) => {
                log::new().syntax(&format!("Determinant::parse({}) 2 -> ({}, {})", old_input, input, success));
                (input, success)
            },
            None => {
                log::new().syntax(&format!("Determinant::parse({}) 3 -> ({}, {})", old_input, old_input, false));
                (old_input, false)
            }
        }
    }
}

impl<'a> Determinant<'a> {
    pub fn get_word(&self) -> String {
        // match self.determinant.left() {
        //     Some(ref defined_articles) => defined_articles.get_word(),
        //     None => match self.determinant.right() {
        //         Some(ref or) => match or.left() {
        //             Some(ref demonstrative_adjectives) => demonstrative_adjectives.get_word(),
        //             None => match or.right() {
        //                 Some(ref or) => match or.left() {
        //                     Some(ref undefined_adjectives) => undefined_adjectives.get_word(),
        //                     None => match or.right() {
        //                         Some(ref or) => match or.left() {
        //                             Some(ref undefined_articles) => undefined_articles.get_word(),
        //                             None => match or.right() {
        //                                 Some(ref or) => match or.left() {
        //                                     Some(ref numerals_adjectives) => numerals_adjectives.get_word(),
        //                                     None => match or.right() {
        //                                         Some(ref or) => match or.left() {
        //                                             Some(ref partial_articles) => partial_articles.get_word(),
        //                                             None => match or.right() {
        //                                                 Some(ref possessive_adjectives) => possessive_adjectives.get_word(),
        //                                                 None => panic!("Invalid determinant type: {}", or)
        //                                             }
        //                                         },
        //                                         None => "".to_string()
        //                                     }
        //                                 },
        //                                 None => "".to_string()
        //                             }
        //                         },
        //                         None => "".to_string()
        //                     }
        //                 },
        //                 None => "".to_string()
        //             }
        //         },
        //         None => "".to_string()
        //     }
        // }
        self.determinant.get()
    }
}

impl<'a> fmt::Display for Determinant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"Determinant\": {{{}}},", self.determinant)
    }
}