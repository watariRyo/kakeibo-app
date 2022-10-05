use serde::{Serialize, Deserialize};
use crate::enumeration::expense_category::ExpenseCategory;
use crate::enumeration::income_category::IncomeCategory;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category {
    Income(IncomeCategory),
    Expense(ExpenseCategory),
}