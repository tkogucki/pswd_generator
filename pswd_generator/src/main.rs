use std::fs::File;
use ascii_converter;
use std::error::Error;
use std::mem::drop;
use csv;
fn main() {
    println!("Running...");
    let char_amount:i32 =  5;
    let mut ascii_range: Vec<u8> = (97..123).collect();
    let mut string_v: Vec<u8> = Vec::new();
    let mut output_vec: Vec<String> = Vec::new();
    let mut counter = char_amount;
    println!("Creating Strings");
    create_string(&ascii_range, &mut string_v, counter, &mut output_vec);
    println!("Strings created");
}

fn create_string(ascii_range: &Vec<u8>,string_v:  &mut Vec<u8>, mut counter: i32, output_vec: &mut Vec<String>) {
    let comparison: i32 = 0;
    if counter > comparison {
        let mut new_range: Vec<u8> = ascii_range.clone();
        // println!("In 1st IF");
        for i in ascii_range.clone() {
            let mut char_: Vec<u8> = vec![i];
            let mut new_string: Vec<u8> = string_v.clone();
            new_string.append(&mut char_);
            let mut updated_counter = counter - 1;
            // println!("{}", i);
            create_string(&new_range, &mut new_string, updated_counter, output_vec);
            drop(updated_counter)
            
        }

    }
        
    if counter == comparison {
        // println!("In 2nd IF");
        for i in ascii_range.clone() {
            let mut prev_string_v = string_v.clone();
            prev_string_v.push(i);
            output_vec.push(ascii_converter::decimals_to_string(&prev_string_v).unwrap());
        }
        
        if let Err(e) = write_to_file("output.csv", output_vec.to_vec()) {
            eprintln!("{}", e)
        }
        drop(output_vec)
        
    }
}


fn write_to_file(path: &str, vector: Vec<String>) -> Result<(), Box<dyn Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;
    let mut count = 1;
    for i in vector {
        writer.write_record([count.to_string(), i]);
        count = count + 1
    }

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}
