//! Uma biblioteca para dividir uma string em uma lista de substrings.
//!
//! ## Sobre Lifetimes no Rust
//! Lifetimes (tempos de vida) são a forma do Rust garantir que referências permaneçam válidas.
//! O parâmetro de lifetime `'haystack` indica que as referências (`&str`) dentro da struct devem
//! viver pelo menos tanto quanto o lifetime `'haystack`. Isso previne referências inválidas (dangling pointers)
//! e garante que os dados referenciados existam enquanto a struct estiver em uso.
//! Em resumo: lifetimes garantem que referências não apontem para dados que já foram destruídos.
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/// Um iterador de string que divide uma string em substrings delimitadas por um delimitador.
#[derive(Debug)]

// A struct armazena referências à string original e o delimitador
// O lifetime 'haystack garante que essas referências permaneçam válidas enquanto a struct existir
// D é um tipo genérico que implementa o trait Delimiter, permitindo flexibilidade no tipo de delimitador
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,  // A parte restante da string que ainda precisa ser processada
                                         // Option permite indicar quando não há mais nada para processar (None)
    delimiter: D,    // O delimitador usado para dividir a string (pode ser &str, char, ou qualquer tipo que implemente Delimiter)
}

impl<'haystack, D> StrSplit<'haystack, D> {
    /// Cria uma nova instância do StrSplit
    /// 
    /// Recebe a string completa (haystack) e o delimitador
    /// O delimitador pode ser qualquer tipo que implemente o trait Delimiter
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self { 
            remainder: Some(haystack),  // Inicializa remainder com a string completa envolvida em Some
            delimiter                   // Armazena o delimitador (pode ser &str, char, etc.)
        }
    }
}

/// Trait que define como encontrar o próximo delimitador em uma string
/// Permite que diferentes tipos (como &str e char) sejam usados como delimitadores
pub trait Delimiter {
    /// Retorna Some((início, fim)) se encontrar o delimitador, onde início e fim são índices na string
    /// Retorna None se não encontrar o delimitador
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// Implementa o trait Iterator para StrSplit
// O where clause especifica que D deve implementar o trait Delimiter
impl<'haystack, D> Iterator for StrSplit<'haystack, D> 
where 
    D: Delimiter {
    type Item = &'haystack str;  // Cada item do iterator é uma fatia da string original
    
    // Retorna o próximo pedaço da string dividido pelo delimitador
    fn next(&mut self) -> Option<Self::Item> {
        // Obtém uma referência mutável ao remainder, retorna None se remainder for None
        // O operador ? faz early return se remainder for None
        let remainder = self.remainder.as_mut()?;
        
        // Tenta encontrar o próximo delimitador usando o trait Delimiter
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            // Se encontrou o delimitador:
            // delim_start é o índice onde o delimitador começa
            // delim_end é o índice onde o delimitador termina
            
            // Pega a substring do início até o delimitador (sem incluí-lo)
            let until_delimiter = &remainder[..delim_start];
            
            // Atualiza remainder para começar após o delimitador
            // delim_end.. pega tudo a partir do fim do delimitador
            *remainder = &remainder[delim_end..];
            
            // Retorna o pedaço encontrado
            Some(until_delimiter)
        } else {
            // Se não encontrou mais delimitadores:
            // take() move o valor de remainder (retorna Some(remainder) e deixa None no lugar)
            // Isso retorna o último pedaço da string e marca remainder como None para próxima iteração
            self.remainder.take()
        }
    }
}

// Implementa Delimiter para &str (string slice)
// Permite usar strings como delimitadores (ex: "abc", " ")
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // s.find(self) procura a primeira ocorrência de self em s
        // Retorna Some(índice) se encontrar, None caso contrário
        s.find(self).map(|start| {
            // Se encontrou, calcula o índice de início e fim
            // start é onde o delimitador começa
            // start + self.len() é onde o delimitador termina
            (start, start + self.len())
        })
    }
}

// Implementa Delimiter para char
// Permite usar caracteres como delimitadores (ex: ' ', ',', 'a')
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // char_indices() retorna um iterator sobre (índice_byte, char) de cada caractere
        // Isso é necessário porque caracteres UTF-8 podem ocupar múltiplos bytes
        s.char_indices()
        // find() procura o primeiro caractere que seja igual a self
        // |(_, c)| c == self compara cada caractere com o delimitador
        .find(|(_, c)| c == self)
        .map(|(start, _)| {
            // Se encontrou, calcula o índice de início e fim
            // start é o índice em bytes onde o caractere começa
            // len_utf8() retorna quantos bytes o caractere ocupa (1 para ASCII, mais para UTF-8)
            (start, start + self.len_utf8())
        })
    }
}

/// Função auxiliar que retorna a substring até a primeira ocorrência de um caractere
#[allow(dead_code)]
fn until_char(s: &str, c: char) -> &str {
    // Cria um novo StrSplit usando o caractere c como delimitador
    StrSplit::new(s, c)
    // next() retorna o primeiro pedaço (até o delimitador)
    .next()
    // expect() desempacota o Option, causando panic se for None
    // Isso garante que o caractere foi encontrado
    .expect("Expected to find a char")
}

#[test]
fn until_char_test() {
    // Testa a função until_char
    // Deve retornar "hell" (tudo antes do primeiro 'o' em "hello world")
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";  // String de teste com espaços como delimitadores
    // Cria o iterator e coleta todos os pedaços em um Vec
    // O tipo do Vec é inferido automaticamente (_)
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    // Verifica se a divisão foi feita corretamente
    // Deve dividir a string nos espaços, resultando em ["a", "b", "c", "d", "e"]
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}