// -------------------CONFIG:-------------------
const CRASH_ON_START: bool = false; // in case you dont want to use it
const DEV_MODE: bool = true; // extra print statements (if thats what you want)

// -------------------CRATES:-------------------
use jwalk::WalkDir; // file directory crate
use rayon::prelude::*;
use std::io;
use std::path::Path; // multithreading crate

// --------------------TODO:--------------------
// 8. Add AI recognition for photos, and text.
// 8. Add the ability for it to run using
// 8.   using Ollama/Groq API keys easily
// 8. LINUX SUPPORT!!!!!!!!!!!!!!!!!!!!!!!!!!!
// 8.
// 8.
// 8.
// 8.
// 8.

fn main() {
    // --------------------VAR.:--------------------
    let mut error_code = 404; // fallback error code
    let mut error_msg = "Item not found";
    //  (error_code, error_msg) = (404, "Cant find it");
    //  eprintln!("[WARNING]: Error {}, {}", error_code, error_msg);

    let target_folder = Path::new("C:\\Users\\sungk\\Downloads\\");
    // --------------------CODE:--------------------

    if CRASH_ON_START {
        (error_code, error_msg) = (0, "Crash on start enabled");
        eprintln!("[ERROR]: Error {}, {}", error_code, error_msg);
        return ();
    }

    // avoid making the var over again in the loop
    // this is also what the user wants to search
    let mut search_query = String::new();

    // whimsy font
    println!(
        r"


   ,d8888b  d8,                d8b    
   88P'    `8P                 88P    
d888888P                      d88  
  ?88'      88b  88bd88b  d888888   d8888b 
  88P       88P  88P' ?8bd8P' ?88  d8P' ?88
 d88       d88  d88   88P88b  ,88b 88b  d88
d88'      d88' d88'   88b`?88P'`88b`?8888P'

"
    );

    println!("Enter the name of your file that you want to search.");
    println!("Use 'exit' to terminate the program.");

    // --------------------MAIN:--------------------
    loop {
        search_query.clear();

        // read user input
        io::stdin()
            .read_line(&mut search_query)
            .expect("Please enter a valid directory!");

        // clean the input
        let cleaned_search = search_query.trim();
        // println!("{}", cleaned_search);

        if cleaned_search.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // get the entries
        let entries: Vec<_> = WalkDir::new(target_folder)
            .skip_hidden(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect();

        // parallel processing
        let matches: Vec<_> = entries
            .par_iter()
            .filter(|entry| {
                // check ONLY files, not directories
                if entry.file_type().is_file() {
                    let file_name = entry.file_name().to_string_lossy();
                    // make it case insensitive
                    file_name
                        .to_lowercase()
                        .contains(&cleaned_search.to_lowercase())
                } else {
                    false
                }
            })
            .map(|entry| entry.path())
            .collect();

        // printing takes more time, so add an option
        if DEV_MODE {
            for path in &matches {
                // having an if statement inside of the for loop is expensive,
                // so do it outside
                println!("Found: {:?}", path);
            }

            println!("Found {} matching file(s).", matches.len());
        } else {
            // NOT dev mode, so dont print it
            if !matches.is_empty() {
                for path in &matches {
                    let mut current_match: Option<String> =
                        Some(path.to_string_lossy().to_string());

                    if let Some(val) = &current_match {
                        if val.to_lowercase().contains(&cleaned_search.to_lowercase()) {
                            println!("Found: {:?}", path);
                        }
                    }
                }
            }
        }

        println!("Done scanning!");
    }
}
