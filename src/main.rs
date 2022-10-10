use self::arquivo::Arquivo;

mod arquivo;
mod model;

fn main() {
    let arquivo = Arquivo::new(include_bytes!("../problemas.toml"));
    arquivo.into_iter_problemas().enumerate().for_each(|(n, problema)| {
        let resposta = problema.resolver().map_or_else(
            || "-- Sem resposta --".to_string(),
            |palavra| palavra.to_string(),
        );
        println!("{:02}: {}", n, resposta);
    });
}
