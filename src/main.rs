mod sender;

fn main() {
    let target = "192.168.55.100:8210";

    match sender::send_entries(target) {
        Ok(()) => println!("Entries imported successfully."),
        Err(e) => eprintln!("Failed to import entries\n\nError: {}", e),
    };
}
