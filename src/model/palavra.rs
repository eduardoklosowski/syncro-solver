use std::fmt;

use super::Letra;

#[derive(Debug, Clone)]
pub struct Palavra {
    letras: Vec<Letra>,
}

impl Palavra {
    pub fn new() -> Self {
        Self { letras: vec![] }
    }

    pub fn adiciona_letra(&self, letra: Letra) -> Self {
        let mut letras = self.letras.clone();
        letras.push(letra);
        Self { letras }
    }
}

impl fmt::Display for Palavra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letras = self
            .letras
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        write!(f, "{}", letras.join(" "))
    }
}
