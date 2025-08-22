use std::collections::{HashMap, LinkedList};

use serde::Deserialize;

use super::{Funcao, Letra, Problema};

/// Estrutura do arquivo com problemas.
#[derive(Debug, Deserialize)]
pub struct Arquivo {
    problemas: Vec<ProblemaDoArquivo>,
}

impl Arquivo {
    pub fn new(conteudo: &[u8]) -> Self {
        toml::from_slice(conteudo).unwrap()
    }

    pub fn into_iter_problemas(self) -> ProblemasIter {
        ProblemasIter {
            problemas: LinkedList::from_iter(self.problemas),
        }
    }
}

/// Iterador de problemas no arquivo.
pub struct ProblemasIter {
    problemas: LinkedList<ProblemaDoArquivo>,
}

impl Iterator for ProblemasIter {
    type Item = Problema;

    fn next(&mut self) -> Option<Self::Item> {
        self.problemas.pop_front().map(|problema| problema.into())
    }
}

/// Estrutura do problema no arquivo.
#[derive(Debug, Deserialize)]
struct ProblemaDoArquivo {
    vertices: usize,
    quadrado: Option<Vec<(usize, usize)>>,
    circulo: Option<Vec<(usize, usize)>>,
    triangulo: Option<Vec<(usize, usize)>>,
}

impl From<ProblemaDoArquivo> for Problema {
    fn from(problema: ProblemaDoArquivo) -> Self {
        let mut funcoes = HashMap::new();
        if let Some(transformacoes) = problema.quadrado {
            funcoes.insert(Letra::Quadrado, Funcao::new(transformacoes));
        }
        if let Some(transformacoes) = problema.circulo {
            funcoes.insert(Letra::Circulo, Funcao::new(transformacoes));
        }
        if let Some(transformacoes) = problema.triangulo {
            funcoes.insert(Letra::Triangulo, Funcao::new(transformacoes));
        }

        Self::new(problema.vertices, funcoes)
    }
}
