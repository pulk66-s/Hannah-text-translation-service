# AST -> LSF translation system

## Introduction

This folder explain all the translation system of the AST to LSF.
When you want to translate a lang to LSF, you need to pass by the AST that represent the french syntax tree.

## How to translate a lang to LSF

Before understanding how to translate a lang to LSF, you need to understand how the AST is structured and you need to know how the LSF is structured.

See here:
- [french AST](./../../Lang/FR/AST.md)
- [LSF](./../../Lang/LSF/README.md)

Now that you understand how the AST and the LSF works, you can translate a 
lang to LSF.

---

We have first in the file [translate.rs](./../../../../src/translation/lsf/translate.rs) the function:

```rust
pub fn lsf_translation(text: Text) -> String {
    translate_text(text).trim().to_string()
}
```
It takes a `Text` and return a `String`. and call the function `translate_text`, trim it and return the `String` object.

---

The `translate_text` function looks like this:

```rust
pub fn translate_text(text: Text) -> String {
    text.phrases.items.iter().map(|phrase| {
        translate_phrase(phrase)
    }).collect::<Vec<String>>().join(" ")
}
```

In the French [AST](./../../Lang/FR/AST.md) you have a `Text` object that contains `Many<Phrase>`, It's an array of `Phrase` object. So we need to iterate over the `Phrase` array and translate each `Phrase` object.

---

The `translate_phrase` function looks like this:

```rust
pub fn translate_phrase(phrase: &Phrase) -> String {
    translate_nominal_group(&phrase.gn) + " " + &(match &phrase.verb.expr {
        Some(expr) => translate_group_verbal(&expr) + " ",
        None => "".to_string()
    }) + &(match &phrase.complement.expr {
        Some(expr) => translate_complement(&expr),
        None => "".to_string()
    })
}
```

It's more interesting, the phrase object looks like this

```rust
pub struct Phrase {
    pub gn: NominalGroup,
    pub verb: Maybe<Verb>,
    pub complement: Maybe<Complement>,
}
```
Nothing complicated, you have a `NominalGroup`, a `Maybe<Verb>` and a `Maybe<Complement>`. The `Maybe` object is an enum that can be `Some` or `None`. So we need to translate the `NominalGroup` and the `Maybe<Verb>` and the `Maybe<Complement>`. When `Maybe` is None, we return an empty string.

---

The `translate_nominal_group` function looks like this:

```rust
pub fn translate_nominal_group(group: &NominalGroup) -> String {
    group.noun.get_word() + " " + &(match &group.adjectives.expr {
        Some(expr) => translate_adjectival_group(&expr),
        None => "".to_string()
    })
}
```

The `NominalGroup` object looks like this:

```rust
pub struct NominalGroup {
    context: Context,
    pub determinant: Maybe<Determinant>,
    pub noun: Noun,
    pub adjectives: Maybe<AdjectivalGroup>,
}
```

The `context` is useless for us, in the function we only get the `noun` and the `adjectives` because in the LSF the `determinant` is useless. So we need to translate the `noun` and the `adjectives`.

And so on..

TODO: continue the explanation