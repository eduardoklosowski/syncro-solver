use std::collections::HashMap;

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

    pub fn opera(&self, valor: usize) -> usize {
        *self.transformacoes.get(&valor).unwrap_or(&valor)
    }
}
