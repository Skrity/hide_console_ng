//! This app will show it's secrets only when launched from console or via a shortcut with argument: "wannasee"

fn main() {
    use std::time::Duration;

    let user_wants_to_see = std::env::args().any(|arg| arg == "wannasee");
    println!("Application is starting; console will be hidden soon");
    std::thread::sleep(Duration::from_secs(1));
    if !user_wants_to_see {
        #[cfg(windows)]
        hide_console_ng::hide_console();
    }
    for _ in 0..5 {
        println!("Doing naughty stuff; hidden from user");
        std::thread::sleep(Duration::from_secs(1));
    }
}
