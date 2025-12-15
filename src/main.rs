pub mod fqchecker;
mod tui;

use fqchecker::fqc::*;

use tokio::sync::watch;

use crate::tui::Interface;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let file_path = std::env::args().nth(1);
    if file_path.is_none() {
        eprintln!("Please provide a valid file path as an argument.");
        std::process::exit(1);
    }
    if let Some(file_path) = file_path {
        let prog_stat = CrrFileProcessInfo::new(file_path.as_str());
        let mut r = CrrFileProcessInfo::new(file_path.as_str());
        let (send_port, receive_port) = watch::channel(prog_stat);
        let mut tui_if = Interface::default();
        tokio::spawn(async move {
            match tui::exec(&mut tui_if, receive_port) {
                Ok(_) => {}
                Err(_) => {
                    panic!("Error in starting terminal interface!!");
                }
            };
        });
        fq_init(&mut r, send_port);
        
    }
    Ok(())
}
