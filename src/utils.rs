use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

pub fn read_csv_data(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(file);
    
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|s| s.to_string()).collect());
    }

    Ok(data)
}
