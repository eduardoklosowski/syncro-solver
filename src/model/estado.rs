use super::Funcao;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Estado {
    vertices: Vec<bool>,
}

impl Estado {
    pub fn new(vertices: usize) -> Self {
        Self {
            vertices: (0..vertices).map(|_| true).collect(),
        }
    }

    pub fn operar(&self, funcao: &Funcao) -> Self {
        let mut novo_estado_vertices: Vec<_> = self.vertices.iter().map(|_| false).collect();

        self.vertices
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .for_each(|origem| {
                let destino = funcao.opera(origem);
                novo_estado_vertices[destino] = true;
            });

        Self {
            vertices: novo_estado_vertices,
        }
    }

    pub fn esta_sincronizado(&self) -> bool {
        self.vertices
            .iter()
            .try_fold(false, |encontrado, &v| {
                if v {
                    if encontrado {
                        None
                    } else {
                        Some(true)
                    }
                } else {
                    Some(encontrado)
                }
            })
            .map_or(false, |_| true)
    }
}
