use std::collections::HashMap;

/// Movimentações realizadas simultâneamento no autômato.
#[derive(Debug)]
pub struct Funcao {
    transformacoes: HashMap<usize, usize>,
}

impl Funcao {
    pub fn new(transformacoes: Vec<(usize, usize)>) -> Self {
        Self {
            transformacoes: HashMap::from_iter(transformacoes),
        }
    }

    /// Retorna para onde o estado do vértice deverá ir depois da operação.
    pub fn opera(&self, valor: usize) -> usize {
        *self.transformacoes.get(&valor).unwrap_or(&valor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_funcao_vazia() {
        let funcao = Funcao::new(vec![]);

        for i in 0..10 {
            assert_eq!(funcao.opera(i), i);
        }
    }

    #[test]
    fn teste_funcao_proximo() {
        let funcao = Funcao::new((0..10).map(|i| (i, i + 1)).collect());

        for i in 0..10 {
            assert_eq!(funcao.opera(i), i + 1);
        }
    }

    #[test]
    fn teste_funcao_anterior() {
        let funcao = Funcao::new((1..=10).rev().map(|i| (i, i - 1)).collect());

        for i in 1..10 {
            assert_eq!(funcao.opera(i), i - 1);
        }
    }

    #[test]
    fn teste_funcao_pares() {
        let funcao = Funcao::new((0..10).filter(|i| i % 2 != 0).map(|i| (i, i + 1)).collect());

        let (pares, impares): (Vec<_>, Vec<_>) = (0..10).partition(|i| i % 2 == 0);
        for i in pares {
            assert_eq!(funcao.opera(i), i);
        }
        for i in impares {
            assert_eq!(funcao.opera(i), i + 1);
        }
    }
}
