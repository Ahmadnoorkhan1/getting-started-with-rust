use std::fs::File;
use std::io::Write;
use std::time:: Instant;
use std::thread;

pub fn blocking_download(urls: Vec<&str>){

    let start = Instant::now();
    let mut handles = vec![];
    for (i, url) in urls.iter().enumerate(){
        let url = url.to_string();
        handles.push(thread::spawn(move || {
            let bytes = reqwest::blocking::get(&url).unwrap().bytes().unwrap();
        let file_name = format!("file_{}.bin", i);
        let mut file = File::create(file_name).unwrap();
        file.write_all(&bytes).unwrap();
        }));
    }

    for h in handles{
        h.join().unwrap();
    }

    println!("Blocking download finished in  {:?}", start.elapsed());


}