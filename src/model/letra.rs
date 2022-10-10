use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
