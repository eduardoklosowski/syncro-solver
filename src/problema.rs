use super::{Estado, Funcao, Letra, Palavra};
use std::collections::{HashMap, HashSet, LinkedList};

/// Autômato e funções que alterar o estado dos vértices.
#[derive(Debug)]
pub struct Problema {
    vertices: usize,
    funcoes: HashMap<Letra, Funcao>,
}

impl Problema {
    pub fn new(vertices: usize, funcoes: HashMap<Letra, Funcao>) -> Self {
        Self { vertices, funcoes }
    }

    /// Tenta sincronizar o autômato, retornando a palavra de sincronismo se possível.
    pub fn sincronizar(&self) -> Option<Palavra> {
        let mut estados_analisados = HashSet::new();
        let mut fila_de_processamento =
            LinkedList::from([(Estado::new(self.vertices), Palavra::new())]);

        while let Some((estado, palavra)) = fila_de_processamento.pop_front() {
            let nova_analise = estados_analisados.insert(estado.clone());
            if !nova_analise {
                continue;
            }

            for (letra, funcao) in self.funcoes.iter() {
                let novo_estado = estado.operar(funcao);
                let nova_palavra = palavra.adiciona_letra(*letra);
                if novo_estado.esta_sincronizado() {
                    return Some(nova_palavra);
                }
                fila_de_processamento.push_back((novo_estado, nova_palavra));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Problema::new(2, [
        (Letra::Quadrado, Funcao::new(vec![(0, 1)])),
    ].into_iter().collect()), Palavra::new().adiciona_letra(Letra::Quadrado))]
    #[case(Problema::new(2, [
        (Letra::Quadrado, Funcao::new(vec![(0, 1), (1, 0)])),
        (Letra::Triangulo, Funcao::new(vec![(1, 0)])),
    ].into_iter().collect()), Palavra::new().adiciona_letra(Letra::Triangulo))]
    #[case(Problema::new(4, [
        (Letra::Quadrado, Funcao::new(vec![(1, 0), (2, 3)])),
        (Letra::Triangulo, Funcao::new(vec![(2, 1)])),
        (Letra::Circulo, Funcao::new(vec![(3, 2)])),
    ].into_iter().collect()), Palavra::new().adiciona_letra(Letra::Circulo).adiciona_letra(Letra::Triangulo).adiciona_letra(Letra::Quadrado))]
    fn teste_resolver(#[case] problema: Problema, #[case] palavra: Palavra) {
        assert_eq!(problema.sincronizar(), Some(palavra));
    }

    #[rstest]
    #[case(Problema::new(2, HashMap::new()))]
    #[case(Problema::new(2, [
        (Letra::Circulo, Funcao::new(vec![(0, 1), (1, 0)])),
    ].into_iter().collect()))]
    #[case(Problema::new(4, [
        (Letra::Circulo, Funcao::new(vec![(1, 0)])),
        (Letra::Triangulo, Funcao::new(vec![(2, 3)])),
    ].into_iter().collect()))]
    fn teste_impossivel_resolver(#[case] problema: Problema) {
        assert_eq!(problema.sincronizar(), None);
    }
}
