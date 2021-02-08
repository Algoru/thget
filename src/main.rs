use clap::{Arg, App};
use regex::Regex;
use serde::Deserialize;
use std::fs;

fn main() {
    let matches = App::new("thget")
        .version("1.0")
        .author("Álvaro Stagg <alvarostagg@protonmail.com>")
        .about("Download 4chan threads")
        .arg(Arg::new("THREAD")
             .about("Thread URL")
             .required(true)
             .index(1))
        .arg(Arg::new("output")
             .about("directory where downloaded content will be placed")
             .short('o')
             .long("output")
             .required(false)
             .takes_value(true))
        .get_matches();

     let thread_url = matches.value_of("THREAD").unwrap();
     
     let thread_re = Regex::new(r"https://boards\.4chan\.org/(?P<board>\S+)/thread/(?P<thread>\d+).*").unwrap();
     let tokens = match thread_re.captures(thread_url) {
         Some(t) => t,
         None => {
             eprintln!("{}: URL isn't valid.", get_program_name());
             std::process::exit(1);
         },
     };

     let board = String::from(&tokens["board"]);
     let thread = &tokens["thread"].parse::<u64>().unwrap();

     let output_dir_path = match matches.value_of("output") {
         Some(output) => String::from(output),
         None => std::env::current_dir().unwrap().into_os_string().into_string().unwrap(),
     };

     let thread_responses = get_thread_responses(&board, thread);
     create_output_dir(&output_dir_path);
     for thread in thread_responses.iter() {
         download_thread_response(thread, &board, output_dir_path.to_owned());
     }
}

fn create_output_dir(output_dir: &String) {
    match fs::create_dir_all(&output_dir) {
        Ok(_) => println!("output directory created"),
        Err(_) => eprintln!("unable to create output directory"),
    }
}

fn download_thread_response(thread: &ThreadResponse, board: &String, output_dir: String) {
    if thread.filename == "" || thread.ext == "" {
        return;
    }

    let download_url = format!("https://i.4cdn.org/{}/{}{}", board, thread.tim, thread.ext);
    println!("downloading file from {}", download_url);
    let mut response = reqwest::blocking::get(&download_url).unwrap();

    if !response.status().is_success() {
        eprintln!("[status:{}] [number:{}] unable to download file", response.status(), thread.number);
        return;
    }
    
    let filename = format!("{}{}", thread.filename, thread.ext);
    let mut file_path = std::path::PathBuf::new();
    file_path.push(&output_dir);
    file_path.push(&filename);

    println!("saving {} into {}", filename, output_dir);

    let mut out_file = fs::File::create(file_path).unwrap();

    match std::io::copy(&mut response, &mut out_file) {
        Err(e) => eprintln!("unable to download {}: {}", filename, e),
        _ => (),
    }
}

#[derive(Deserialize, Debug)]
struct Thread {
    posts: Vec<ThreadResponse>
}

#[derive(Deserialize, Debug)]
struct ThreadResponse {
    #[serde(rename = "no")]
    number: u64,
    #[serde(default)]
    filename: String,
    #[serde(default)]
    ext: String,
    #[serde(default)]
    tim: u64,
}

fn get_thread_responses(board: &String, thread: &u64) -> Vec<ThreadResponse> {
    let api_thread_url = format!("https://a.4cdn.org/{}/thread/{}.json", board, thread);

    let response = reqwest::blocking::get(&api_thread_url).unwrap();
    if !response.status().is_success() {
        eprintln!("[status:{}] unable to get thread responses", response.status());
        return vec![];
    }
    let thread_responses: Thread = response.json().unwrap();

    thread_responses.posts
}

fn get_program_name<'a>() -> String {
    let name = match std::env::current_exe() {
        Ok(prog_name) => String::from(prog_name.to_str().unwrap_or("thget")),
        Err(_) => String::from("thget"),
    };
    
    name
}
