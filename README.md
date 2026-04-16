# 📖 Agenda de Contatos em Rust

Uma aplicação simples de linha de comando (CLI) desenvolvida em **Rust** para gerenciamento de contatos. Este projeto demonstra conceitos fundamentais da linguagem, como manipulação de strings, structs, vetores, ownership/borrowing e pattern matching.

## ✨ Funcionalidades

O sistema apresenta um menu interativo com as seguintes opções:

1. **Adicionar contato:** Salva um novo contato com Nome, E-mail e Telefone.
2. **Listar contatos:** Exibe todos os contatos cadastrados em formato de lista numerada.
3. **Buscar contato:** Permite procurar um contato específico pelo nome (ignorando letras maiúsculas e minúsculas).
4. **Remover contato:** Busca e exclui um contato específico da agenda.
5. **Sair:** Encerra a aplicação de forma segura.

## 🚀 Pré-requisitos

Para rodar este projeto, você precisará ter o **Rust** e o gerenciador de pacotes **Cargo** instalados em sua máquina.

Caso não os tenha, instale através do [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 🛠️ Como executar

1. Clone este repositório ou crie uma pasta para o projeto.
2. Navegue até o diretório do projeto no seu terminal.
3. Se você criou o projeto usando `cargo new agenda`, basta executar o comando abaixo:

```bash
cargo run
```

O Cargo irá compilar o código e iniciar a aplicação no seu terminal.

## 🧠 Estrutura do Código

O projeto é construído em torno de duas estruturas principais (`Structs`):

* `Contato`: Define as propriedades de um contato individual (`nome`, `email` e `telefone`).
* `Agenda`: Gerencia uma coleção (`Vec`) de contatos e implementa os métodos para manipular esses dados (`nova`, `adicionar`, `listar`, `buscar` e `remover`).

O tratamento de entradas do usuário é centralizado na função auxiliar `ler_input`, garantindo que o fluxo principal no `main` permaneça limpo e focado no controle do menu através da estrutura `match`.

## 🤝 Contribuindo

Sinta-se à vontade para fazer um *fork* deste projeto, abrir *issues* ou enviar *pull requests* com melhorias, como:
* Salvar e carregar os contatos em um arquivo `.txt` ou `.json`.
* Adicionar validação para o formato de e-mail e telefone.
* Melhorar a interface visual do terminal.
