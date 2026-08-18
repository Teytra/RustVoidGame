use std::io::{self, Write};

pub fn wait_for_user_input() {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}
pub fn get_user_input() -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let girdi = loop {
        buf.clear();

        print!("->");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buf)?;

        if buf.trim().is_empty() {
            eprintln!("Geçerli bir şey girin.");
            continue;
        }

        break buf;
    };
    Ok(girdi)
}