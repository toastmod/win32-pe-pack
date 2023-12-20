use std::{error::Error, fs::{self, copy}};

use goblin::pe::PE;

const DLLPATH: &str = "C:\\Windows\\System32\\";

fn main() -> Result<(), Box<dyn Error>> {
    Ok(match std::env::args().nth(1) {
        Some(arg) => {
            let paths: Vec<String> = std::env::var_os("PATH")
                .expect("PATH not found :(")
                .into_string()
                .expect("Error parsing PATH contents")
                .split(';')
                .map(|x|String::from(x))
                .collect();

            let bytes = fs::read(&arg)?;
            
            let mut dllbuf = String::new();
            let mut copybuf = String::new();
            let mut pastebuf = String::new();
            let mut upper_pastebuf = String::new();
            let mut lower_pastebuf = String::new();
            let mut dirbuf = String::from(".\\");
            dirbuf.push_str(&arg);
            dirbuf = dirbuf.replace(".exe", "-packed");
            
            // Create package dir if it doesn't already exist 
            if fs::read_dir(&dirbuf).is_err() {
                fs::create_dir(&dirbuf);
            }

            let pe = PE::parse(&bytes)?;
            
            //Fetch all DLLs
            for dll in pe.libraries {
                println!("{}", dll);

                // Prepare DLL string
                dllbuf.clear();
                dllbuf.push_str("\\");
                dllbuf.push_str(dll);
                
                // Prepare DLL destination filepath string
                pastebuf.clear();
                pastebuf.push_str(&dirbuf);
                pastebuf.push_str(&dllbuf);

                // Prepare upper and lowercase
                upper_pastebuf.push_str(&dirbuf);
                upper_pastebuf.push_str(&dllbuf.to_uppercase());
                upper_pastebuf.push_str(&dirbuf);
                lower_pastebuf.push_str(&dllbuf.to_lowercase());

                let mut found = false;
                
                // Scan PATH
                'searchpath: for path in &paths {

                    // Prepare DLL source filepath string 
                    copybuf.clear();
                    copybuf.push_str(path);
                    copybuf.push_str(&dllbuf);

                    for paste in [
                        &pastebuf,
                        &upper_pastebuf,
                        &lower_pastebuf
                    ] {
                        // Attempt copy
                        match fs::copy(&copybuf, paste) {
    
                            // Finish scan when copy succeeds
                            Ok(_) => {
                                found = true;
                                break 'searchpath;
                            },
    
                            Err(_) => {
                                // Try next DLL filepath
                                found = false;
                                continue 'searchpath;
                            },
    
                        };                        
                    }


                }

                // If the DLL was not found throw an error
                if !found {
                    println!("Error: {} not found!", dll)
                }else{
                    // Finally, if all goes well, copy the executable.
                    pastebuf.clear();
                    pastebuf.push_str(&dirbuf);
                    pastebuf.push_str("\\");
                    pastebuf.push_str(
                        &arg
                        .replace(".\\", "")
                        .replace("./", "")
                        .replace("\\", "")
                        .replace("/", "")
                    );
                    fs::copy(&arg, &pastebuf);
                }
            }
        }
        None => println!("Usage: xpack [FILENAME]"),
    })
}
