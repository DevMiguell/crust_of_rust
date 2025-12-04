# StrSplit

Iterador customizado em Rust para dividir strings usando delimitadores flexíveis (`&str` ou `char`).

## Funcionalidades

- Divide strings com delimitadores `&str` ou `char`
- Implementa `Iterator`
- Suporta UTF-8 corretamente
- Gerencia lifetimes de forma segura

## Exemplos

```rust
use strsplit::StrSplit;

// Delimitador char
let text = "a,b,c";
let parts: Vec<_> = StrSplit::new(text, ',').collect();
assert_eq!(parts, vec!["a", "b", "c"]);

// Delimitador string
let text = "hello---world";
let parts: Vec<_> = StrSplit::new(text, "---").collect();
assert_eq!(parts, vec!["hello", "world"]);
```

## Estrutura

- `StrSplit<'haystack, D>`: Struct principal com `remainder` e `delimiter`
- `Delimiter`: Trait implementado para `&str` e `char`

## Conceitos Demonstrados

- Lifetimes (`'haystack`)
- Generics (`D`)
- Traits (`Delimiter`, `Iterator`)
- UTF-8 handling

## Testes

```bash
cargo test
```

