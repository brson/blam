#![allow(unused)]

fn main() {
    println!("Hello, world!");
}

use std::collections::BTreeMap;
use float_ord::FloatOrd;

type String = std::string::String;
type Integer = i64;
type Float = f64;
type Date = chrono::Date<chrono::Utc>;

struct Table {
    name: String,
    rows: usize,
    columns: Vec<Column>,
    name_to_idx: BTreeMap<String, usize>,
    unique_indexes: BTreeMap<usize, UniqueIndex>,
}

#[derive(Clone)]
struct Column {
    name: String,
    data: ColumnData,
    unique_key: bool,
    foreign_key: Option<(String, String)>,
}

#[derive(Clone)]
enum ColumnData {
    Integer(Vec<Integer>),
    Float(Vec<Float>),
    String(Vec<String>),
    Date(Vec<Date>),
}

#[derive(Clone)]
enum UniqueIndex {
    Integer(BTreeMap<Integer, usize>),
    Float(BTreeMap<FloatOrd<Float>, usize>),
    String(BTreeMap<String, usize>),
    Date(BTreeMap<Date, usize>),
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
    fn from_columns(name: String, columns: Vec<Column>) -> Table {
        let mut rows = None;
        let mut name_to_idx = BTreeMap::new();
        let mut unique_indexes = BTreeMap::new();

        for (column_idx, column) in columns.iter().enumerate() {
            let new_rows = column.len();
            if let Some(rows) = rows {
                assert_eq!(rows, new_rows);
            }
            rows = Some(new_rows);
            name_to_idx.insert(column.name.clone(), column_idx);

            if column.unique_key {
                let index = match &column.data {
                    ColumnData::Integer(ref key_column) => {
                        let mut index = BTreeMap::new();
                        for (row_idx, key) in key_column.iter().enumerate() {
                            assert!(index.insert(key.clone(), row_idx).is_none());
                        }
                        UniqueIndex::Integer(index)
                    }
                    ColumnData::Float(ref key_column) => {
                        let mut index = BTreeMap::new();
                        for (row_idx, key) in key_column.iter().enumerate() {
                            assert!(index.insert(FloatOrd(key.clone()), row_idx).is_none());
                        }
                        UniqueIndex::Float(index)
                    }
                    ColumnData::String(ref key_column) => {
                        let mut index = BTreeMap::new();
                        for (row_idx, key) in key_column.iter().enumerate() {
                            assert!(index.insert(key.clone(), row_idx).is_none());
                        }
                        UniqueIndex::String(index)
                    }
                    ColumnData::Date(ref key_column) => {
                        let mut index = BTreeMap::new();
                        for (row_idx, key) in key_column.iter().enumerate() {
                            assert!(index.insert(key.clone(), row_idx).is_none());
                        }
                        UniqueIndex::Date(index)
                    }
                };
                unique_indexes.insert(column_idx, index);
            }
        }

        let rows = rows.unwrap_or(0);

        Table {
            name,
            rows,
            columns,
            name_to_idx,
            unique_indexes,
        }
    }
}

impl Table {
    fn join(&self, name: String, other: &Table, crit: JoinCriteria) -> Table {
        let join_self_column_idx = *self.name_to_idx.get(&crit.self_column).expect("self_column");
        let join_other_column_idx = *self.name_to_idx.get(&crit.other_column).expect("other_column");
        let join_self_column = self.columns.get(join_self_column_idx).expect("self_idx");
        let join_other_column = self.columns.get(join_other_column_idx).expect("other_idx");

        assert!(join_other_column.unique_key);

        let mut rows = 0;
        let mut old_columns = Vec::new();
        let mut new_columns = Vec::new();
        let mut name_to_idx = BTreeMap::new();
        let mut unique_indexes = BTreeMap::new();

        for column in &self.columns {
            let column_name = format!("{}.{}", self.name, column.name);
            let new_column = Column {
                name: column_name,
                data: column.data.clone(),
                unique_key: column.unique_key,
                foreign_key: column.foreign_key.clone(),
            };

            old_columns.push(new_column);

            if column.unique_key {
                panic!()
            }

            if column.name == join_self_column.name {
                panic!()
            }
        }

        old_columns.extend(new_columns.into_iter());

        let columns = old_columns;

        Table {
            name,
            rows,
            columns,
            name_to_idx,
            unique_indexes,
        }
    }

    fn select(&self, crit: SelectCriteria) -> Table { panic!() }
}
