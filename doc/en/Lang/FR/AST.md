# AST

---

## Introduction<a name="introduction"></a>

This file contains all the explanation of the AST (Abstract Syntax Tree) in Rust.

## Explanation<a name="explanation"></a>

The code works in two parts:

- Lang -> AST
- AST -> LSF

We are using the AST to make an intermediate language. We are converting the language to the AST, then we are converting the AST to LSF. In this way it can be use to convert any language to LSF and it's the best way to represent a language.

---

### Behavior<a name="behavior"></a>

We are using a Rust trait named `Expr`, that looks like this:

```rust
pub trait Expr {
    fn parse<'a>(&mut self, input: &'a str) -> (&'a str, bool);
    fn new(context: Context) -> Self;
}
```

We have two different functions:

```rust
fn new(context: Context) -> Self;
```

This function is used to create a new instance of the struct with a `Context`. The `Context` is used to store the datas that we need to parse the code.
e.g.:

verbs.json:

```json
{
  "attrape": {
    "verb": "attraper",
    "tense": "present"
  },
  "prête": {
    "verb": "prêter",
    "tense": "present"
  }
}
```

The second function is:

```rust
fn parse<'a>(&mut self, input: &'a str) -> (&'a str, bool);
```

This function is used to parse the code. It takes a `&str` as input and returns a `&str` and a `bool`. The `&str` is the rest of the code that we need to parse and the `bool` is `true` if the parsing is successful and `false` if it's not.

---

### Basic Schema<a name="basic-schema"></a>

Each struct that implements the `Expr` trait has a `parse` function. The final AST looks like this right now (this is not finished and it will be updated once it will be tested):

```rust
- Text            // To test
  - Many<Phrase>

- Phrase          // To test
  - Subject
  - Verb
  - Complement

- Complement  // To test
  - Todo

- Verb
  - Word

- Subject
    - Or<
      NominalSubject,
      PronounSubject,
    >

- NominalSubject
    - Maybe<Determinant>
    - Noun

- PronounSubject
    - Pronoun

- Pronoun
    - Or<
      NominalSubject, SubjectPronoun
    >

- Determinant
    - Or<
      DefinedArticles,
      Or<
        DemonstrativeAdjectives,
        Or<
          UndefinedAdjectives,
        Or<
          UndefinedArticles,
          Or<
            NumeralsAdjectives,
            Or<
              PartialArticles,
              PossessiveAdjectives
              >
            >
          >
        >
      >
    >

- Word  // To test
  - Many<
    Or<
      Alphanumeric,
      SpecialChar
      >
    >
  >

- Alphanumeric // To test
  - a-zA-Z0-9

- SpecialChar // To test
  - "'" | "-"

- SubjectPronoun
    - PronounType

enum PronounType {
    FirstPersonSingular,
    SecondPersonSingular,
    ThirdPersonSingular,
    FirstPersonPlural,
    SecondPersonPlural,
    ThirdPersonPlural,
    Unknown,
}
```