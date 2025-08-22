use self::arquivo::Arquivo;
use self::estado::Estado;
use self::funcao::Funcao;
use self::letra::Letra;
use self::palavra::Palavra;
use self::problema::Problema;

mod arquivo;
mod estado;
mod funcao;
mod letra;
mod palavra;
mod problema;

fn main() {
    let arquivo = Arquivo::new(include_bytes!("../problemas.toml"));
    arquivo
        .into_iter_problemas()
        .enumerate()
        .for_each(|(n, problema)| {
            let resposta = problema.sincronizar().map_or_else(
                || "-- Sem resposta --".to_string(),
                |palavra| palavra.to_string(),
            );
            println!("{n:02}: {resposta}");
        });
}
