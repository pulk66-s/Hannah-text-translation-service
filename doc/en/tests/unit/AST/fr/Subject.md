# Test for Subject AST

---

## Introduction

Explanation of tests for `Subject` AST. See documentation [here](../../../../Lang/FR/Subject.md) to see what is a `Subject` AST.

### Pronouns

Datas:

- Je:
  ```rust
  vec![
      "je".to_string(),
      "jE".to_string(),
      "Je".to_string(),
      "JE".to_string(),
      "J'".to_string(),
      "j'".to_string(),
      " je ".to_string(),
      " jE ".to_string(),
      " Je ".to_string(),
      " JE ".to_string(),
      " J' ".to_string(),
      " j' ".to_string(),
  ]
  ```
- Tu:
  ```rust
  vec![
      "tu".to_string(),
      "tU".to_string(),
      "Tu".to_string(),
      "TU".to_string(),
      "T'".to_string(),
      "t'".to_string(),
      " tu ".to_string(),
      " tU ".to_string(),
      " Tu ".to_string(),
      " TU ".to_string(),
      " T' ".to_string(),
      " t' ".to_string()
  ];
  ```
- Il:
  ```rust
  vec![
      "il".to_string(),
      "iL".to_string(),
      "Il".to_string(),
      "IL".to_string(),
      " il ".to_string(),
      " iL ".to_string(),
      " Il ".to_string(),
      " IL ".to_string(),
  ];
  ```
- Elle:
  ```rust
  vec![
      "elle".to_string(),
      "eLLe".to_string(),
      "Elle".to_string(),
      "ELLE".to_string(),
      " elle ".to_string(),
      " eLLe ".to_string(),
      " Elle ".to_string(),
      " ELLE ".to_string(),
  ];
  ```
- Nous:
  ```rust
  vec![
      "nous".to_string(),
      "nOUs".to_string(),
      "Nous".to_string(),
      "NOUS".to_string(),
      " nous ".to_string(),
      " nOUs ".to_string(),
      " Nous ".to_string(),
      " NOUS ".to_string(),
  ];
  ```
- Vous:
  ```rust
  vec![
      "vous".to_string(),
      "vOUs".to_string(),
      "Vous".to_string(),
      "VOUS".to_string(),
      " vous ".to_string(),
      " vOUs ".to_string(),
      " Vous ".to_string(),
      " VOUS ".to_string(),
  ];
  ```
- Ils:
  ```rust
  vec![
      "ils".to_string(),
      "iLS".to_string(),
      "Ils".to_string(),
      "ILS".to_string(),
      " ils ".to_string(),
      " iLS ".to_string(),
      " Ils ".to_string(),
      " ILS ".to_string(),
  ];
  ```
- Elles:
  ```rust
  vec![
      "elles".to_string(),
      "eLLEs".to_string(),
      "Elles".to_string(),
      "ELLES".to_string(),
      " elles ".to_string(),
      " eLLEs ".to_string(),
      " Elles ".to_string(),
      " ELLES ".to_string(),
  ];
  ```

### Pronouns

#### With Determinants:

```rust
let phrase: String = String::from("le chat");
let phrase: String = String::from("chat");
```