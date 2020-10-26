#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

use std::collections::BTreeMap;

type String = std::string::String;
type Integer = i64;
type Float = f64;
type Date = chrono::Date<chrono::Utc>;

struct Table {
    rows: usize,
    columns: Vec<Column>,
    name_to_idx: BTreeMap<String, usize>,
}

struct Column {
    name: String,
    data: ColumnData,
}

enum ColumnData {
    Integer(Vec<Integer>),
    Float(Vec<Float>),
    String(Vec<String>),
    Date(Vec<String>),
}

struct JoinCriteria {
}

struct SelectCriteria {
}

impl Table {
    fn join(&self, other: &Table, crit: JoinCriteria) -> Table { panic!() }
    fn select(&self, crit: SelectCriteria) -> Table { panic!() }
}
