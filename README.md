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
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
