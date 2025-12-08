# VecMac

Macro customizada em Rust para criar `Vec` de forma conveniente, similar ao `vec!` padrão.

## Para que serve

A macro `avec!` permite criar vetores de forma mais concisa:

```rust
// Em vez de:
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);

// Você pode fazer:
let v = avec![1, 2, 3];
```

## Funcionalidades

- Cria `Vec` vazio: `avec![]`
- Cria `Vec` com elementos: `avec![1, 2, 3]`
- Suporta vírgula trailing: `avec![1, 2, 3,]`

## Exemplos

```rust
use vecmac::avec;

// Vec vazio
let empty: Vec<i32> = avec![];

// Vec com elementos
let numbers = avec![1, 2, 3, 4, 5];

// Vec com vírgula trailing
let items = avec![
    "item1",
    "item2",
    "item3",
];
```

## Conceitos Demonstrados

- **Macros declarativas** (`macro_rules!`)
- **Repetição de padrões** (`$(...)*`)
- **Captura de expressões** (`$element:expr`)
- **Padrões opcionais** (`$(,)?`)

## Testes

```bash
cargo test
```



