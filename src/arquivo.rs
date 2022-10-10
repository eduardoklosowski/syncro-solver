use std::collections::{HashMap, LinkedList};

use serde::Deserialize;

use super::model;

#[derive(Debug, Deserialize)]
pub struct Arquivo {
    problemas: Vec<Problema>,
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

pub struct ProblemasIter {
    problemas: LinkedList<Problema>,
}

impl Iterator for ProblemasIter {
    type Item = model::Problema;

    fn next(&mut self) -> Option<Self::Item> {
        self.problemas.pop_front().map(|problema| problema.into())
    }
}

#[derive(Debug, Deserialize)]
struct Problema {
    vertices: usize,
    quadrado: Option<Vec<(usize, usize)>>,
    circulo: Option<Vec<(usize, usize)>>,
    triangulo: Option<Vec<(usize, usize)>>,
}

impl From<Problema> for model::Problema {
    fn from(problema: Problema) -> Self {
        let mut funcoes = HashMap::new();
        if let Some(transformacoes) = problema.quadrado {
            funcoes.insert(model::Letra::Quadrado, model::Funcao::new(transformacoes));
        }
        if let Some(transformacoes) = problema.circulo {
            funcoes.insert(model::Letra::Circulo, model::Funcao::new(transformacoes));
        }
        if let Some(transformacoes) = problema.triangulo {
            funcoes.insert(model::Letra::Triangulo, model::Funcao::new(transformacoes));
        }

        Self::new(problema.vertices, funcoes)
    }
}
