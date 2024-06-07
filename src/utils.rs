pub mod utils {
    pub fn clear_screen() {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    }
    pub fn pause() {
        let mut new_line = String::new();
        _ = std::io::stdin().read_line(&mut new_line);
    }
}
