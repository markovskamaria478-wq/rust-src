enum LogEntry {
    Info(String),
    Warning(u8),
    Error(Option<String>),
    Heartbeat,
}

fn main() {
    let entries = [
        LogEntry::Info(String::from("")),
        LogEntry::Warning(202),
        LogEntry::Error(None),
        LogEntry::Error(Some(String::from("example1.ext"))),
        LogEntry::Error(Some(String::from("example.ext"))),
        LogEntry::Heartbeat,
    ];

    println!("Source file name with errors:");
    let mut count = 0;
    for entry in &entries {
        if let LogEntry::Error(Some(name)) = entry {
            count += 1;
            println!("{name}");
        }
    }
    println!("Errors with a source file {}", count);
}
