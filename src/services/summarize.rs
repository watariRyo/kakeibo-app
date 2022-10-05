use std::{collections::{BTreeSet, BTreeMap}};
use chrono::{Datelike, NaiveDate};
use crate::models::item::Item;
use crate::services;

pub fn run(file_path: &str) {
    println!("家計簿の集計を行います");
    let data = services::io_service::read_data_or_panic(file_path);

    let target_dates: BTreeSet<NaiveDate> = get_target_dates(&data);
    let mut result_table: BTreeMap<NaiveDate, i32> = BTreeMap::new();

    for date in target_dates {
        let filtered_data = get_filtered_date(&data, date);
        let sum = summarize_data(&filtered_data);
        result_table.insert(date, sum);
    }
    print_table(result_table)
}

fn get_target_dates(data: &Vec<Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data.iter().map(|item| {
        item.get_first_day()
    }).collect();
    target_dates
}

fn get_filtered_date(data: &Vec<Item>, filter_date: NaiveDate) -> Vec<&Item> {
    let filtered_data: Vec<&Item> = data.iter().filter(|item| {
        (item.get_year() == filter_date.year()) && (item.get_month() == filter_date.month())
    }).collect();
    filtered_data
}

fn summarize_data(data: &Vec<&Item>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += item.get_price_for_summary();
    }
    sum
}

fn format_date(date: NaiveDate) -> String {
    format!("{}/{}", date.year(), date.month())
}

fn format_price(price: i32) -> String {
    if price > 0 {
        format!("+{}", price)
    } else {
        format!("-{}", price)
    }
}

fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
    for result in result_table {
        let date = format_date(result.0);
        let price = format_price(result.1);
        println!("{}の収支は{}円でした", date, price);
    }
}

#[cfg(test)]
mod summarize_test {
    use std::collections::{BTreeSet};
    use chrono::NaiveDate;
    use crate::enumeration::category::Category;
    use crate::enumeration::expense_category::ExpenseCategory;
    use crate::enumeration::income_category::IncomeCategory;
    use crate::models::item::Item;
    use crate::services::summarize::*;

    fn get_test_data() -> Vec<Item>{
        vec![
            Item::new(
                "新年会".to_string(),
                Category::Expense(ExpenseCategory::Food),
                5000,
                NaiveDate::from_ymd(2022, 1, 10)
            ),
            Item::new(
                "給料".to_string(),
                Category::Income(IncomeCategory::Salary),
                300000,
                NaiveDate::from_ymd(2022, 1, 20)
            ),
            Item::new(
                "旅行".to_string(),
                Category::Expense(ExpenseCategory::Hobby),
                100000,
                NaiveDate::from_ymd(2022, 1, 30)
            ),
            Item::new(
                "外食".to_string(),
                Category::Expense(ExpenseCategory::Food),
                3000,
                NaiveDate::from_ymd(2022, 2, 15)
            ),
            Item::new(
                "歓迎会".to_string(),
                Category::Expense(ExpenseCategory::Other),
                10000,
                NaiveDate::from_ymd(2022, 4, 15)
            ),
        ]
    }

    #[test]
    fn test_get_target_dates() {
        let test_data = get_test_data();
        let mut expected = BTreeSet::new();

        expected.insert(NaiveDate::from_ymd(2022, 1, 1));
        expected.insert(NaiveDate::from_ymd(2022, 2, 1));
        expected.insert(NaiveDate::from_ymd(2022, 4, 1));

        assert_eq!(get_target_dates(&test_data), expected);
    }
}