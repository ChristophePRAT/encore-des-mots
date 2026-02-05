use std::error::Error;
use std::fs::File;
use std::path::Path;

use csv::StringRecord;

pub struct Record {
    pub id: u32,
    pub sw: String,
    pub pl: String,
    pub en: String,
    pub fr: String,
    pub la: String,
    pub nl: String,
    pub bs: String,
    pub ca: String,
    pub es: String,
    pub sk: String,
    pub pt: String,
    pub hr: String,
    pub it: String,
    pub sl: String,
    pub de: String,
}

pub fn parse_csv_line(line: StringRecord) -> Option<Record> {
    let fields: Vec<&str> = line.iter().collect();
    if fields.len() != 16 {
        return None; // Invalid line
    }

    Some(Record {
        id: fields[0].parse().ok()?,
        sw: fields[1].to_string(),
        pl: fields[2].to_string(),
        en: fields[3].to_string(),
        fr: fields[4].to_string(),
        la: fields[5].to_string(),
        nl: fields[6].to_string(),
        bs: fields[7].to_string(),
        ca: fields[8].to_string(),
        es: fields[9].to_string(),
        sk: fields[10].to_string(),
        pt: fields[11].to_string(),
        hr: fields[12].to_string(),
        it: fields[13].to_string(),
        sl: fields[14].to_string(),
        de: fields[15].to_string(),
    })
}

pub fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(parsed_record) = parse_csv_line(record) {
            records.push(parsed_record);
        }
    }
    Ok(records)
}
pub fn print_record(record: &Record) {
    println!(
        "ID: {}, Sw: {}, Pl: {}, En: {}, Fr: {}, La: {}, Nl: {}, Bs: {}, Ca: {}, Es: {}, Sk: {}, Pt: {}, Hr: {}, It: {}, Sl: {}, De: {}",
        record.id,
        record.sw,
        record.pl,
        record.en,
        record.fr,
        record.la,
        record.nl,
        record.bs,
        record.ca,
        record.es,
        record.sk,
        record.pt,
        record.hr,
        record.it,
        record.sl,
        record.de
    );
}
