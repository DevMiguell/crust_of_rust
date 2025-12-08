// Exporta a macro para que possa ser usada em outros crates
#[macro_export]
macro_rules! avec {
    // Caso 1: Macro chamada sem argumentos - retorna Vec vazio
    () => {
        Vec::new()
    };
    // Caso 2: Macro chamada com um ou mais elementos
    // $($element:expr), + : captura um ou mais elementos separados por vírgula
    // $(,)? : vírgula trailing opcional (permite vírgula no final)
    ($($element:expr), + $(,)?) => {{
        // Cria um Vec mutável vazio
        let mut vs = Vec::new();
        // Repete o push para cada elemento capturado
        // $(vs.push($element);)* : repete a operação para cada elemento
        $(vs.push($element);)*
        // Retorna o Vec preenchido
        vs
    }};
}

// Teste: macro sem argumentos deve retornar Vec vazio
#[test]
fn empty_vec() {
    let v: Vec<u32> = avec![];
    assert!(v.is_empty());
}

// Teste: macro com um único elemento
#[test]
fn single() {
    let v: Vec<u32> = avec![42];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 42);
}

// Teste: macro com múltiplos elementos
#[test]
fn double() {
    let v: Vec<u32> = avec![42, 43];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 42);
    assert_eq!(v[1], 43);
}

// Teste: macro com vírgula trailing (vírgula no final)
#[test]
fn trailing() {
    let _: Vec<&'static str> = avec![
        "dasdanjshaskdhasjkdhasdjkhasdjkashdjkashdajks",
        "dasdanjshaskdhasjkdhasdjkhasdjkashdjkashdajks",
        "dasdanjshaskdhasjkdhasdjkhasdjkashdjkashdajks",
        "dasdanjshaskdhasjkdhasdjkhasdjkashdjkashdajks",
        "dasdanjshaskdhasjkdhasdjkhasdjkashdjkashdajks",
    ];
}