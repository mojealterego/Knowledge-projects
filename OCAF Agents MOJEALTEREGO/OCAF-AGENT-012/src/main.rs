use std::io::{Cursor, Write};
use zip::{write::FileOptions, ZipWriter};
use zip_warden_agent::agent::audit_bytes;
use zip_warden_agent::security::load_config;
fn main() {
    let c = load_config(include_str!("../config/agent.toml")).expect("invalid agent identity");
    let mut out = Cursor::new(Vec::<u8>::new());
    {
        let mut z = ZipWriter::new(&mut out);
        z.start_file("demo.txt", FileOptions::default()).unwrap();
        z.write_all(b"offline fixture").unwrap();
        z.finish().unwrap();
    }
    let r = audit_bytes(&out.into_inner(), false).unwrap();
    println!(
        "{} | {} | members={} | semantic_review={}",
        c.identity.unique_name,
        c.identity.serial_number,
        r.member_count,
        r.semantic_review_performed
    );
}
