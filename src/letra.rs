use std::fmt;

/// Nome de operações no autômato.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Letra {
    Quadrado,
    Circulo,
    Triangulo,
}

impl fmt::Display for Letra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Quadrado => write!(f, "□"),
            Self::Circulo => write!(f, "○"),
            Self::Triangulo => write!(f, "△"),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use self::Letra::*;
    use super::*;

    #[rstest]
    #[case(Quadrado, "□")]
    #[case(Circulo, "○")]
    #[case(Triangulo, "△")]
    fn teste_to_string(#[case] letra: Letra, #[case] esperado: &str) {
        assert_eq!(letra.to_string(), esperado);
    }

    #[rstest]
    #[case(Quadrado, Quadrado)]
    #[case(Circulo, Circulo)]
    #[case(Triangulo, Triangulo)]
    fn teste_igual(#[case] a: Letra, #[case] b: Letra) {
        assert_eq!(a, b);
    }

    #[rstest]
    #[case(Quadrado, Circulo)]
    #[case(Circulo, Triangulo)]
    #[case(Triangulo, Quadrado)]
    fn teste_diferente(#[case] a: Letra, #[case] b: Letra) {
        assert_ne!(a, b);
    }
}
