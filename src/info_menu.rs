pub mod info_menu {
    use std::io::{stdin, stdout, Write};
    use utils::utils::{clear_screen, pause};

    use crate::{images::images, utils};

    pub fn main() {
        loop {
            clear_screen();
            println!("{}", images::ARTS[2]);
            println!("          ----------------------------------------");
            println!("          |    Hostname:                         |");
            println!("          |    IPv4:                             |");
            println!("          |    MAC:                              |");
            println!("          |    OS:                               |");
            println!("          |                                      |");
            println!("          |                                      |");
            println!("          |                                      |");
            println!("          |                                      |");
            println!("          |                        0) Voltar     |");
            println!("          ----------------------------------------");
            print!("          Selecione uma opção: >> ");
            let _ = stdout().flush();

            let mut input = String::new();
            _ = stdin().read_line(&mut input);
            let input: &str = input.as_str().trim();

            match input {
                "0" => {
                    break;
                }
                _ => {
                    println!("Opção inválida!");
                    pause();
                }
            }
        }
    }
}
