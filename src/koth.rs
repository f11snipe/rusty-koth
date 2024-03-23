use serde_json;
use std::{
    fs,
    thread,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    time::Duration,
    collections::HashMap,
};

pub fn watch_king(king_file: &String, data_file: &String, tick_points: u32, tick_interval: u64) {
    let data_json = fs::read_to_string(data_file).unwrap_or_else(|_| String::from("{}"));
    let mut scoreboard: HashMap<String, u32> = serde_json::from_str(data_json.as_str()).unwrap();

    loop {
        let king = read_king(king_file);
        let numlines = king.lines().count();

        if king.as_str() != "" && numlines == 1 {
            let e: &mut u32 = scoreboard.entry(king).or_insert(0);
            *e += tick_points;
            let total: u32 = scoreboard.iter().map(|r| r.1).sum();
            let winning = scoreboard.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
            let mut hash_vec: Vec<(&String, &u32)> = scoreboard.iter().collect();
            hash_vec.sort_by(|a, b| b.1.cmp(a.1));

            if total % 10 == 0 {
                println!("Winning: {}", winning.0);
                println!("Current: {}", read_king(king_file));
                println!("{:?}", &winning);
                dbg!(&scoreboard);
                let json = serde_json::to_string(&scoreboard).unwrap();
                if let Err(e) = fs::write(Path::new(data_file), json) {
                    println!("error: {:?}", e);
                }
            }
        }
        thread::sleep(Duration::from_millis(tick_interval));
    }
}

pub fn listen_http(bind_addr: &String, king_file: &String, data_file: &String) {
    let listener = TcpListener::bind(bind_addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, data_file, king_file);
    }
}

fn line_to_words(line: &str) -> Vec<String> {
    line.split_whitespace().map(str::to_string).collect()
}

fn read_data(data_file: &String) -> String {
    if Path::new(data_file).exists() {
        let contents = fs::read_to_string(data_file).expect("Failed to read data file");
        return contents;
    }
    return String::new();
}

fn read_king(king_file: &String) -> String {
    if Path::new(king_file).exists() {
        let contents = fs::read_to_string(king_file).expect("Failed to read king file");
        return contents.trim().to_string();
    }
    return String::new();
}

fn handle_connection(mut stream: TcpStream, data_file: &String, king_file: &String) {
    let buf_reader = BufReader::new(&mut stream);
    let req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let parts = line_to_words(req[0].as_str());

    if parts.len() != 3 {
        let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        return
    }

    let method = &parts[0];
    let route = &parts[1];
    // let version = &parts[2];

    if method != "GET" {
        let response = "HTTP/1.1 420 NOPE\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        return
    }

    if route == "/data" {
        let data = read_data(data_file);
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", data);
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    let king = read_king(king_file);
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", king);
    stream.write_all(response.as_bytes()).unwrap();
}
