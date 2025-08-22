# Syncro Solver

Projeto implementando uma heurística que resolve os problemas do [Syncro](https://mathspp.com/syncro). Essa heurística foi pensada por mim, adaptando os conhecimentos que adquiri depois de resolver os problemas do jogo de forma a ser executado em um computador. Essa implementação tem o objetivo de treinar a utilização de estrutura de dados em [Rust](https://www.rust-lang.org/).

[Vídeo da MathGurl explicando como o jogo funciona](https://www.youtube.com/watch?v=iXgm0qmP3cw)

## Ambiente de Desenvolvimento

O ambiente de desenvolvimento desse projeto pode ser criado dentro de um contêiner com [Development Containers](https://containers.dev/). Para isso é necessário utilizar algum editor com suporte, como o [Visual Studio Code](https://code.visualstudio.com/) com a extensão [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers). Neste caso basta pedir para o VS Code abrir o projeto dentro de um contêiner que toda a configuração e ferramentas necessárias para o desenvolvimento serão instaladas automaticamente.

Para facilitar a execução de comandos neste projeto, o [GNU Make](https://www.gnu.org/software/make/) foi configurado, bastante executar `make <comando-desejado>`. Para listar os principais comandos basta executar:

```sh
make help
```
