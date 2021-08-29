use std::fs;
use ascii_converter;
use std::error::Error;
use std::mem::drop;
use csv;
fn main() {
    println!("Running...");

    // Note char_amount should be indexed from zero
    let char_amount:i32 =  5;

    // Creating saving counter as saving operation was taking significant computation time
    let mut saving_counter = 0;

    // currently only lower and upper case letters, struggles to run 6 characters without overflow error.  Needs to be fixed
    let mut ascii_range: Vec<u8> = (97..123).collect();
    let mut ascii_range_2: Vec<u8> = (65..91).collect();
    ascii_range.append(&mut ascii_range_2);
    let mut string_v: Vec<u8> = Vec::new();
    let mut output_vec: Vec<String> = Vec::new();
    let mut counter = char_amount;
    println!("Creating Strings");
    saving_counter = create_string(&ascii_range, &mut string_v, counter, &mut output_vec, saving_counter);
    println!("Strings created");

    println!("{} : Saving file to reduce buffer", saving_counter);
    if let Err(e) = write_to_file("output.csv", output_vec.to_vec()) {
        eprintln!("{}", e);
        // Notifiy user of save
    }
}

fn create_string(ascii_range: &Vec<u8>,string_v:  &mut Vec<u8>, 
    mut counter: i32, output_vec: &mut Vec<String>, mut saving_counter: i32) -> i32 {
    // Match type of counter variable
    let comparison: i32 = 0;

    // Create for loop through ascii range
    // Decrement the counter value and recursively call function again
    // new range is created to allow for passing to recursive along with updated counter
    if counter > comparison {
        let mut new_range: Vec<u8> = ascii_range.clone();
        // println!("In 1st IF");
        for i in ascii_range.clone() {
            let mut char_: Vec<u8> = vec![i];
            let mut new_string: Vec<u8> = string_v.clone();
            new_string.append(&mut char_);
            let mut updated_counter = counter - 1;
            saving_counter = saving_counter + 1;
            // println!("{}", i);
            saving_counter = create_string(&new_range, &mut new_string, updated_counter, output_vec, saving_counter);
            drop(updated_counter);
            
        }
        

    }
  
    let increment_save: i32 = 25000000;
    if counter == comparison {
        // For final character only.  Pushes to output vector all the combinations 
        for i in ascii_range.clone() {
            let mut prev_string_v = string_v.clone();
            prev_string_v.push(i);
            output_vec.push(ascii_converter::decimals_to_string(&prev_string_v).unwrap());
            saving_counter = saving_counter + 1;

            // Writing to file to ease memory burden and prevent linux KILL from initiating
            if saving_counter % increment_save == comparison {
                // Notifiy user of save
                println!("{} : Saving file to reduce buffer", saving_counter);

                if let Err(e) = write_to_file("output.csv", output_vec.to_vec()) {
                    eprintln!("{}", e); 
                }
                output_vec.clear()           
            }
        }
        
    }
    return saving_counter;
}


// Code from CSV Writer example : Need to work on understanding this
fn write_to_file(path: &str, vector: Vec<String>) -> Result<(), Box<dyn Error>> {
    //Create File
    let mut file = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(path)
    .unwrap();
    
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_writer(file);
    let mut count = 1;
    for i in vector {
        writer.write_record([i]);
        count = count + 1
    }

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}
