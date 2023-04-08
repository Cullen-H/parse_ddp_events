use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let mut map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

    let file = File::open("events.json")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let json: serde_json::Value = serde_json::from_str(&line).expect("Invalid json");
        *map.entry(json["event_type"].to_string().to_owned()).or_default() += 1;
    }

    let events_file = File::create("output/output.json").unwrap();

    serde_json::to_writer(events_file, &map).unwrap();

    Ok(())
}
