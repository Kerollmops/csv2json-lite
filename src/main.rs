use std::io;
use std::error::Error;
use json::object::Object;

fn try_main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let headers = rdr.headers()?.clone();

    print!("[");

    let mut iter = rdr.into_records().peekable();
    while let Some(result) = iter.next() {
        let record = result?;
        let mut object = Object::new();

        for (i, field) in record.iter().enumerate() {
            let header = headers.get(i).unwrap();
            object.insert(&header, json::from(field));
        }

        if iter.peek().is_some() {
            println!("{},", object.dump());
        } else {
            print!("{}", object.dump());
        }
    }

    println!("]");

    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
    }
}
