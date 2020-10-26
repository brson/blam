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

#[derive(Clone)]
struct Column {
    name: String,
    data: ColumnData,
}

#[derive(Clone)]
enum ColumnData {
    Integer(Vec<Integer>),
    Float(Vec<Float>),
    String(Vec<String>),
    Date(Vec<String>),
}

struct JoinCriteria {
    self_column: String,
    other_column: String,
}

struct SelectCriteria {
}

impl Column {
    fn len(&self) -> usize { panic!() }
}

impl Table {
    fn from_columns(columns: Vec<Column>) -> Table {
        let mut rows = None;
        let mut name_to_idx = BTreeMap::new();
        for (idx, column) in columns.iter().enumerate() {
            let new_rows = column.len();
            if let Some(rows) = rows {
                assert_eq!(rows, new_rows);
            }
            rows = Some(new_rows);
            name_to_idx.insert(column.name.clone(), idx);
        }

        let rows = rows.unwrap_or(0);

        Table {
            rows,
            columns,
            name_to_idx,
        }
    }
}

impl Table {
    fn join(&self, other: &Table, crit: JoinCriteria) -> Table {
        let join_self_column_idx = *self.name_to_idx.get(&crit.self_column).expect("self_column");
        let join_other_column_idx = *self.name_to_idx.get(&crit.other_column).expect("other_column");
        let join_self_column = self.columns.get(join_self_column_idx).expect("self_idx");
        let join_other_column = self.columns.get(join_other_column_idx).expect("other_idx");

        panic!()
    }

    fn select(&self, crit: SelectCriteria) -> Table { panic!() }
}
