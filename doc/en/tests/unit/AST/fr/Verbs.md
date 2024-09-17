# Test for Verb AST

---

## Introduction

Explanation of tests for `Verb` AST. See documentation [here](../../../../Lang/FR/Verb.md) to see what is a `Verb` AST.

All the tests are done with the verb "manger" (to eat). Because it's a regular verb, it's easier to test. If it works it means that everything else works because it's the same for all the verbs.

---

## Datas

```rust
vec![
    "mange".to_string(),
    "manges".to_string(),
    "mange".to_string(),
    "mangeons".to_string(),
    "mangez".to_string(),
    "mangent".to_string(),
]
```