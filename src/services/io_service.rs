use std::fs::File;
use std::io::Write;
use std::io::{BufReader, Error};
use crate::models;

pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::item::Item> {
    let file: Result<File, Error> = File::open(file_path);
    match file {
        Ok(f) => {
            let buf_reader: BufReader<File> = BufReader::new(f);
            serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました")
        },
        Err(_) => {
            println!("新規ファイルを作成します");
            Vec::new()
        }
    }
}

pub fn read_data_or_panic(file_path: &str) -> Vec<models::item::Item> {
    let file: File = File::open(file_path).expect("ファイルをオープンできませんでした");
    let buf_reader: BufReader<File> = BufReader::new(file);
    let data: Vec<_> = serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました");

    if data.len() == 0 {
        panic!("データが存在しません");
    }
    data
}

pub fn write_to_json(data: &Vec<models::item::Item>, file_path: &str) {
    let json_data = serde_json::to_string_pretty(data).expect("JSONへのシリアライズに失敗しました");
    let mut file = File::create(file_path).expect("書き込みファイルのオープンに失敗しました");
    writeln!(file, "{}", json_data).expect("ファイルの書き込みに失敗しました");
    println!("項目の登録が完了しました");
}