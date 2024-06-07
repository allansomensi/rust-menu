pub mod snmp_menu {
    use std::io::{stdin, stdout, Write};
    use utils::utils::{clear_screen, pause};

    use crate::{images::images, printers_menu, utils};

    pub fn main() {
        loop {
            clear_screen();
            println!("{}", images::ARTS[3]);
            println!("----------------------------------------");
            println!("|    1) Impressoras                    |");
            println!("|    2) Switchs                        |");
            println!("|    3) Câmeras                        |");
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
                    printers_menu::printers_menu::main();
                }
                _ => {
                    println!("Opção inválida!");
                    pause();
                }
            }
        }
    }
}
