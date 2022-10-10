use std::collections::{HashMap, HashSet, LinkedList};

use super::{Estado, Funcao, Letra, Palavra};

#[derive(Debug)]
pub struct Problema {
    vertices: usize,
    funcoes: HashMap<Letra, Funcao>,
}

impl Problema {
    pub fn new(vertices: usize, funcoes: HashMap<Letra, Funcao>) -> Self {
        Self { vertices, funcoes }
    }

    pub fn resolver(&self) -> Option<Palavra> {
        let mut estados_analisados = HashSet::new();
        let mut fila_de_processamento =
            LinkedList::from([(Estado::new(self.vertices), Palavra::new())]);

        while let Some((estado, palavra)) = fila_de_processamento.pop_front() {
            let estado_nao_analisado = estados_analisados.insert(estado.clone());
            if !estado_nao_analisado {
                continue;
            }

            if estado.esta_sincronizado() {
                return Some(palavra);
            }

            for (letra, funcao) in self.funcoes.iter() {
                let novo_estado = estado.operar(funcao);
                let nova_palavra = palavra.adiciona_letra(letra.clone());
                fila_de_processamento.push_back((novo_estado, nova_palavra));
            }
        }
        None
    }
}
