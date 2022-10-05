use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenseCategory {
    Food,
    Hobby,
    Other,
}
