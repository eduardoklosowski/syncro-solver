use std::fmt;

use super::Letra;

/// Sequência de [`Letras`], ou seja, uma cadência de operações no autômato.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palavra {
    letras: Vec<Letra>,
}

impl Palavra {
    pub fn new() -> Self {
        Self { letras: vec![] }
    }

    /// Cria uma nova palavra com [`letra`] ao final.
    pub fn adiciona_letra(&self, letra: Letra) -> Self {
        let mut letras = self.letras.clone();
        letras.push(letra);
        Self { letras }
    }
}

impl From<Vec<Letra>> for Palavra {
    fn from(value: Vec<Letra>) -> Self {
        Palavra { letras: value }
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Letra::*;
    use super::*;

    #[rstest]
    #[case(Palavra::new(), "")]
    #[case(Palavra::from(vec![Quadrado]), "□")]
    #[case(Palavra::from(vec![Quadrado, Circulo]), "□ ○")]
    #[case(Palavra::from(vec![Quadrado, Circulo, Triangulo]), "□ ○ △")]
    #[case(Palavra::from(vec![Circulo, Triangulo, Quadrado]), "○ △ □")]
    fn teste_to_string(#[case] palavra: Palavra, #[case] esperado: String) {
        assert_eq!(palavra.to_string(), esperado);
    }

    #[test]
    fn teste_adiciona_letra() {
        let palavra = Palavra::from(vec![Quadrado]);
        assert_eq!(
            palavra.adiciona_letra(Quadrado),
            Palavra::from(vec![Quadrado, Quadrado])
        );
        assert_eq!(
            palavra.adiciona_letra(Circulo),
            Palavra::from(vec![Quadrado, Circulo])
        );
        assert_eq!(
            palavra.adiciona_letra(Triangulo),
            Palavra::from(vec![Quadrado, Triangulo])
        );

        let palavra = Palavra::new()
            .adiciona_letra(Quadrado)
            .adiciona_letra(Circulo)
            .adiciona_letra(Triangulo)
            .adiciona_letra(Quadrado)
            .adiciona_letra(Quadrado);
        assert_eq!(
            palavra,
            Palavra::from(vec![Quadrado, Circulo, Triangulo, Quadrado, Quadrado])
        );
    }
}
