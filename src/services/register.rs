
use std::io;
use std::str::FromStr;
use chrono::NaiveDate;
use crate::services::validate::input_validator::InputValidator;
use crate::models::item::Item;
use crate::services::io_service;

pub fn run(file_path: &str) {
    println!("収支の登録を行います");
    let register_type = input_register_type();
    let name = input_name();
    let category_type = input_category_type(register_type);
    let price = input_price();
    let date = input_date();
    let category = Item::get_category(register_type, category_type);

    let item = Item::new(name, category, price, date);
    println!("{:?}", &item);

    let mut data =  io_service::read_data_or_create_new_data(file_path);
    data.push(item);
    io_service::write_to_json(&data, file_path);
}

fn input_register_type() -> u8 {
    println!("登録種別を入力してください（0：収入、1：支出）");
    let mut register_type = String::new();
    io::stdin().read_line(&mut register_type).expect("登録種別の入力に失敗しました");
    let register_type = register_type.trim().parse().expect("登録種別は数値で入力してください");
    InputValidator::validate_register_type(register_type);
    register_type
}

fn input_name() -> String {
    println!("品目名を入力してください");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("品目名の入力に失敗しました");
    name.trim().to_string()
}

fn input_category_type(register_type: u8) -> u8 {
    println!("カテゴリを入力してください");
    if register_type == 0 {
        println!("0：給与、1：ボーナス、2：その他");
    } else {
        println!("0：食費、1：趣味、2：その他");
    }
    let mut category_type = String::new();
    io::stdin().read_line(&mut category_type).expect("カテゴリ種別の入力に失敗しました");
    let category_type = category_type.trim().parse().expect("カテゴリは数値で入力してください");
    InputValidator::validate_category_type(register_type, category_type);
    category_type
}

fn input_price() -> u32 {
    println!("金額を入力してください");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("金額の入力に失敗しました");
    price.trim().parse().expect("金額は数値で入力してください")
}

fn input_date() -> NaiveDate {
    println!("日付を入力してください（yyyy-mm-dd）");
    let mut date = String::new();
    io::stdin().read_line(&mut date).unwrap();
    NaiveDate::from_str(&date).expect("日付はyyyy-mm-ddの形式で入力してください")
}

