pub mod printers_menu {
    use crate::{images::images, utils};
    use std::io::{stdin, stdout, Write};
    use utils::utils::{clear_screen, pause};

    pub fn main() {
        loop {
            clear_screen();
            println!("{}", images::ARTS[0]);
            println!("         Painel Impressoras (SNMP)");
            println!("----------------------------------------");
            println!("|    1) Verificar status geral         |");
            println!("|    2) Monitor de recursos            |");
            println!("|    3) Lista de dispostivos           |");
            println!("|                                      |");
            println!("|                                      |");
            println!("|                                      |");
            println!("|                                      |");
            println!("|                                      |");
            println!("|                        0) Voltar     |");
            println!("----------------------------------------");
            print!("Selecione uma opção: >> ");
            let _ = stdout().flush();

            let mut input = String::new();
            _ = stdin().read_line(&mut input);
            let input: &str = input.as_str().trim();

            match input {
                "0" => {
                    break;
                }
                "1" => {
                    continue;
                }
                _ => {
                    println!("Opção inválida!");
                    pause();
                }
            }
        }
    }
}
