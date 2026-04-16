use std::io::{self, Write};
struct Contato {
    nome: String,
    email: String,
    telefone: String,
}
struct Agenda {
    contatos: Vec<Contato>,
}
impl Agenda {
    fn nova() -> Agenda{ // Cria uma nova agenda e o vetor de contatos
        Agenda{
            contatos: Vec::new(), 
        }
    }
    fn adicionar(&mut self, nome: String, email: String, telefone: String) { //chama uma ref mutavel de agenda e adiciona nv contact nela
        let contato = Contato { nome, email, telefone };
        self.contatos.push(contato);
    }
    fn listar(&self) { // pega uma referencia de self e intera sobre os contatos do vetor junto de um indice 
        if self.contatos.is_empty() {
            println!("Nenhum contato cadastrado.");
            return;
        }
        for (i, contato) in self.contatos.iter().enumerate() {
            println!("{}. {} | {} | {}", i + 1, contato.nome, contato.email, contato.telefone);

        }
    }
    fn buscar(&self, nome: &str) -> Option<&Contato> {
        for contato in &self.contatos {
            if contato.nome.eq_ignore_ascii_case(nome) {
                return Some(contato);
            }
        }
        None
    }
    fn remover(&mut self, nome: &str) {
        let indice = self.contatos.iter().position(|c| {
            c.nome.eq_ignore_ascii_case(nome)
        });
        match indice {
            Some(i) => {
                self.contatos.remove(i);
                println!("Contato removido");
            },
            None => println!("Não foi encontrado."),
        }
    }

}
fn ler_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().expect("Falha ao limpar o buffer");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler o input.");
    input.trim().to_string()
}
fn main() {
    let mut agenda = Agenda::nova();
    loop {
        println!("\n=== AGENDA ===");
        println!("1. Adicionar contato");
        println!("2. Listar contatos");
        println!("3. Buscar contato");
        println!("4. Remover");
        println!("5. Sair");

        let opcao = ler_input("Escolha sua opção:");

        match opcao.as_str() {
            "1" => {
                let nome = ler_input("Nome:");
                let email = ler_input("Email:");
                let telefone = ler_input("Telefone:");
                agenda.adicionar(nome, email, telefone);
                println!("Contato adicionado.");
            }
            "2" => agenda.listar(),
            "3" => {
                let nome = ler_input("Nome para buscar:");
                match agenda.buscar(&nome){
                    Some(c) => println!("Achado: {} | {} | {}", c.nome, c.email, c.telefone),
                    None => println!("Contato não encontrado"),
                }
            }
            "4" => {
                let nome = ler_input("Nome para remover: ");
                agenda.remover(&nome);
            }
            "5" => {
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Input invalido");
            }
        }

    }
}