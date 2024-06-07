mod images;
mod info_menu;
mod printers_menu;
mod snmp_menu;
mod utils;

use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Write};
use utils::utils::{clear_screen, pause};
fn main() {
    loop {
        clear_screen();

        let greeting_arts = images::images::ARTS;
        let mut rng = rand::thread_rng();
        let mut arts_vec: Vec<&str> = greeting_arts.iter().cloned().collect();
        arts_vec
            .retain(|&x| x != greeting_arts[0] && x != greeting_arts[2] && x != greeting_arts[3]);
        arts_vec.shuffle(&mut rng);
        let selected_art = arts_vec.pop().unwrap_or("Nenhuma arte disponível.");

        println!("{}", selected_art);

        print!("Selecione uma opção: >> ");
        let _ = stdout().flush();

        let mut input = String::new();
        _ = stdin().read_line(&mut input);
        let input: &str = input.as_str().trim();

        match input {
            "0" => {
                println!("Você escolheu sair!");
                pause();
                break;
            }
            "i" => {
                info_menu::info_menu::main();
                continue;
            }
            "admin" => {
                println!("Admin");
                pause();
                continue;
            }
            "1" => {
                snmp_menu::snmp_menu::main();
                continue;
            }
            _ => {
                println!("Opção inválida!");
                pause();
            }
        }
    }
}
