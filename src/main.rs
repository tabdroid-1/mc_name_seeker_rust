use rand::Rng;
use std::io::prelude::*;
use std::io::BufReader; 
use std::io::BufRead; 
use std::fs::File;
use std::fs; 
use std::io; 
use std::{iter, thread, time};
use std::path::Path;

fn main() {

    let mut lines: Vec<String> = Vec::new();
    let mut name_lenght = String::new();
    let mut delay_lenght = String::new();
    let mut loop_count = String::new();
    let mut times_looped: usize = 0;
    let file_name = "names.txt";

    // Set name lenght. If input is not a number, smaller than 3 or bigger than 16, program will panic.
        println!("How long name should be? (minimum 3) >> ");
        io::stdin()
            .read_line(&mut name_lenght)
            .expect("Failed to read line");
        let name_lenght: usize = name_lenght.trim().parse().expect("Please type a number!");
        if name_lenght < 3 || name_lenght > 16 {
            panic!("Name lenght can not be lower than 3 or bigger than 16");
        }
    // -----------------------------

    // Set delay lenght. If input is not a number, smaller than 0 or bigger than 99999, program will panic.
        println!("How long delay between each call should be? (500ms or longer is recomended. Else api may refuse to give data until few minutes passes) >> ");
        io::stdin()
            .read_line(&mut delay_lenght)
            .expect("Failed to read line");
        let delay_lenght: usize = delay_lenght.trim().parse().expect("Please type a number!");
    // -----------------------------

    // Set delay lenght. If input is not a number, smaller than 0 or bigger than 99999, program will panic.
        println!("How many times program should check name? (0 for infinite loop) >> ");
        io::stdin()
            .read_line(&mut loop_count)
            .expect("Failed to read line");
        let loop_count: usize = loop_count.trim().parse().expect("Please type a number!");
    // -----------------------------
    
    println!("-----------------------------------------------------------");
    
    if loop_count == 0 {

        // Program loops forever until program is terminated.
        loop {
            let api = "https://api.mojang.com/users/profiles/minecraft/".to_owned();
            let name = random_string_generator(name_lenght);

            let full_url = format!("{}{}",api, name);
            let data: &str = &call(&&full_url.to_string()).unwrap();



            // println!("Name: {name}");
            // println!("Full url: {full_url}");
            // println!("{data}");

            if data.contains("errorMessage") {
                println!("Name {name} is available!");

                // to check if name.txt file exists
                let mut rs: bool = true;
                rs = Path::new("names.txt").exists();
                // --------------------------------
                
                // reads content of file and saves to vector to add another name then to save to file again (to counter 4 XD)
                if rs == true{
                    
                    lines = file_to_vec(file_name.to_string()).unwrap();
                    lines.push(name.clone());

                    fs::write(file_name, lines.join("\n")).expect("");
                    
                }
                else{
                    match File::create(file_name){
                        Ok(file) => println!("{:?}", file),
                        Err(_) => println!("Unable to create the file: '{}'", file_name)
                    }

                    lines = file_to_vec(file_name.to_string()).unwrap();
                    lines.push(name);

                    fs::write(file_name, lines.join("\n")).expect("");
                }
                // -----------------------------------------------------------------------------------------------------------

                println!("-----------------------------------------------------------");
            }
            

            thread::sleep(time::Duration::from_millis(delay_lenght.try_into().unwrap()));
        }
        // ---------------------------------------------------

    }
    else if loop_count > 0 {
        // Program loops forever until program is terminated.
        while times_looped < loop_count  {
            let api = "https://api.mojang.com/users/profiles/minecraft/".to_owned();
            let name = random_string_generator(name_lenght);

            let full_url = format!("{}{}",api, name);
            let data: &str = &call(&&full_url.to_string()).unwrap();

            // println!("Name: {name}");
            // println!("Full url: {full_url}");
            // println!("{data}");

            if data.contains("errorMessage") {
                println!("Name {name} is available!");

                // to check if name.txt file exists
                let mut rs: bool = true;
                rs = Path::new("names.txt").exists();
                // --------------------------------
                
                // reads content of file and saves to vector to add another name then to save to file again (to counter 4 XD)
                if rs == true{
                    
                    lines = file_to_vec(file_name.to_string()).unwrap();
                    lines.push(name.clone());

                    fs::write(file_name, lines.join("\n")).expect("");
                    
                }
                else{
                    match File::create(file_name){
                        Ok(file) => println!("{:?}", file),
                        Err(_) => println!("Unable to create the file: '{}'", file_name)
                    }

                    lines = file_to_vec(file_name.to_string()).unwrap();
                    lines.push(name);

                    fs::write(file_name, lines.join("\n")).expect("");
                }
                println!("-----------------------------------------------------------");
            }

            thread::sleep(time::Duration::from_millis(delay_lenght.try_into().unwrap()));
            // ---------------------------------------------------
            times_looped += 1;
        }
    }



    
}

// Gets json data from mojang api
#[tokio::main]
async fn call(url: &String) -> Result<String, reqwest::Error> {
    let call = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(call)
}
// --------------------------------

// Generates random string to search name
fn random_string_generator(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}


fn file_to_vec(filename: String) -> io::Result<Vec<String>> { 
    let file_in = fs::File::open(filename)?; 
    let file_reader = BufReader::new(file_in); 
    Ok(file_reader.lines().filter_map(io::Result::ok).collect()) 
}