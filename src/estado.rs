use super::Funcao;

/// Estado dos vértices do autômato.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Estado {
    vertices: Vec<bool>,
}

impl Estado {
    pub fn new(vertices: usize) -> Self {
        Self {
            vertices: vec![true; vertices],
        }
    }

    /// Retorna qual o estado do autômato após a [`funcao`] ser aplicado ao seu estado.
    pub fn operar(&self, funcao: &Funcao) -> Self {
        let mut novo_estado_vertices: Vec<_> = vec![false; self.vertices.len()];

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

    /// Retorna se o autômato está resolvido.
    pub fn esta_sincronizado(&self) -> bool {
        self.vertices
            .iter()
            .try_fold(false, |encontrado, &v| {
                if v {
                    if encontrado { None } else { Some(true) }
                } else {
                    Some(encontrado)
                }
            })
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use rand::{rng, seq::SliceRandom};
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_operar_funcao_impar() {
        let numero_vertices = 14;
        let funcao = Funcao::new(
            (0..numero_vertices / 2)
                .map(|i| (i * 2, i * 2 + 1))
                .collect(),
        );

        let estado = Estado::new(numero_vertices).operar(&funcao);

        assert_eq!(
            estado.vertices,
            (0..numero_vertices)
                .map(|i| (i % 2) != 0)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_operar_funcao_par() {
        let numero_vertices = 14;
        let funcao = Funcao::new(
            (0..numero_vertices / 2)
                .map(|i| (i * 2 + 1, i * 2))
                .collect(),
        );

        let estado = Estado::new(numero_vertices).operar(&funcao);

        assert_eq!(
            estado.vertices,
            (0..numero_vertices)
                .map(|i| (i % 2) == 0)
                .collect::<Vec<_>>()
        );
    }

    #[rstest]
    #[case(4)]
    #[case(5)]
    #[case(8)]
    #[case(10)]
    fn test_sincronismo(#[case] numero_vertices: usize) {
        let mut estado = Estado::new(numero_vertices);
        let mut vertices: Vec<_> = (0..numero_vertices).collect();
        vertices.shuffle(&mut rng());

        for &vertice in vertices.iter().take(vertices.len() - 1) {
            assert!(!estado.esta_sincronizado());
            estado.vertices[vertice] = false;
        }
        assert!(estado.esta_sincronizado());
    }
}
