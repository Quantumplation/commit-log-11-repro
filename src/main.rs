extern crate commitlog;

use commitlog::*;
use commitlog::message::*;


fn main() {

    let opts = LogOptions::new("log");
    let mut log = CommitLog::new(opts).unwrap();

    println!("Last Offset: {:?}", log.last_offset());
    let messages = log.read(0, ReadLimit::default()).unwrap();
    for msg in messages.iter() {
        println!("{} - {}", msg.offset(), String::from_utf8_lossy(msg.payload()));
    }

    log.append_msg("Test").unwrap();;
    log.flush().unwrap();
}
