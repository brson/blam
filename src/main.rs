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

type TableName = std::string::String;
type ColumnName = std::string::String;

struct Table {
    name: TableName,
    rows: usize,
    columns: Vec<Column>,
    name_to_idx: BTreeMap<ColumnName, usize>,
    unique_indexes: BTreeMap<usize, UniqueIndex>,
}

#[derive(Clone)]
struct Column {
    name: ColumnName,
    data: ColumnData,
    unique_key: bool,
    foreign_key: Option<(TableName, ColumnName)>,
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
    new_table_name: TableName,
    self_column: ColumnName,
    other_column: ColumnName,
    other_table_rename: Option<TableName>,
}

struct SelectCriteria {
    new_table_name: TableName,
    columns: Vec<ColumnName>,
}

impl Column {
    fn len(&self) -> usize {
        match self.data {
            ColumnData::Integer(ref v) => v.len(),
            ColumnData::Float(ref v) => v.len(),
            ColumnData::String(ref v) => v.len(),
            ColumnData::Date(ref v) => v.len(),
        }
    }
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
    /// Join with another table by matching self foreign key to other unique key
    fn join(&self, other: &Table, crit: JoinCriteria) -> Table {
        let join_self_column_idx = *self.name_to_idx.get(&crit.self_column).expect("self_column");
        let join_other_column_idx = *self.name_to_idx.get(&crit.other_column).expect("other_column");
        let join_self_column = self.columns.get(join_self_column_idx).expect("self_idx");
        let join_other_column = self.columns.get(join_other_column_idx).expect("other_idx");

        assert!(join_self_column.foreign_key.is_some());
        assert_eq!(join_self_column.foreign_key.as_ref().unwrap().0,
                   other.name);
        assert_eq!(join_self_column.foreign_key.as_ref().unwrap().1,
                   join_other_column.name);
        assert!(join_other_column.unique_key);

        let rows = self.rows;
        let mut old_columns = Vec::new();
        let mut new_columns = Vec::new();
        let mut name_to_idx = BTreeMap::new();
        let mut unique_indexes = BTreeMap::new();

        // Copy columns from self
        for (column_idx, column) in self.columns.iter().enumerate() {
            let column_name = format!("{}.{}", self.name, column.name);
            let new_column = Column {
                name: column_name.clone(),
                data: column.data.clone(),
                unique_key: column.unique_key,
                foreign_key: column.foreign_key.clone(),
            };

            old_columns.push(new_column);
            name_to_idx.insert(column_name, column_idx);
        }

        // Copy indexes from self
        unique_indexes.extend(self.unique_indexes.clone().into_iter());

        // Create new columns from other
        let other_name = crit.other_table_rename.as_ref().unwrap_or(&other.name);
        for (orig_column_idx, column) in other.columns.iter().enumerate() {
            let new_column_idx = self.columns.len() + orig_column_idx;
            let column_name = format!("{}.{}", other_name, column.name);
            let data = match column.data {
                ColumnData::Integer(_) => ColumnData::Integer(Vec::new()),
                ColumnData::Float(_) => ColumnData::Float(Vec::new()),
                ColumnData::String(_) => ColumnData::String(Vec::new()),
                ColumnData::Date(_) => ColumnData::Date(Vec::new()),
            };
            let new_column = Column {
                name: column_name.clone(),
                data,
                unique_key: false,
                foreign_key: column.foreign_key.clone(),
            };

            new_columns.push(new_column);
            name_to_idx.insert(column_name, new_column_idx);
        }

        // A helper for copying a row from other to new columns
        let mut copy_row = |row| {
            for (column_idx, column) in other.columns.iter().enumerate() {
                let mut new_column = new_columns.get_mut(column_idx).expect("column");
                match new_column.data {
                    ColumnData::Integer(ref mut dest) => {
                        let data: &Integer = match column.data {
                            ColumnData::Integer(ref source) => source.get(row).expect("row"),
                            _ => unreachable!(),
                        };
                        dest.push(data.clone());
                    }
                    ColumnData::Float(ref mut dest) => {
                        let data: &Float = match column.data {
                            ColumnData::Float(ref source) => source.get(row).expect("row"),
                            _ => unreachable!(),
                        };
                        dest.push(data.clone());
                    }
                    ColumnData::String(ref mut dest) => {
                        let data: &String = match column.data {
                            ColumnData::String(ref source) => source.get(row).expect("row"),
                            _ => unreachable!(),
                        };
                        dest.push(data.clone());
                    }
                    ColumnData::Date(ref mut dest) => {
                        let data: &Date = match column.data {
                            ColumnData::Date(ref source) => source.get(row).expect("row"),
                            _ => unreachable!(),
                        };
                        dest.push(data.clone());
                    }
                }
            }
        };

        // Iterate through the foreign key and copy every row from other
        let index = other.unique_indexes.get(&join_other_column_idx).expect("index");
        match join_self_column.data {
            ColumnData::Integer(ref keys) => {
                let index = match index {
                    UniqueIndex::Integer(ref i) => i,
                    _ => unreachable!(),
                };
                for key in keys {
                    let row = *index.get(key).expect("foreign key");
                    copy_row(row);
                }
            }
            ColumnData::Float(ref keys) => {
                let index = match index {
                    UniqueIndex::Float(ref i) => i,
                    _ => unreachable!(),
                };
                for key in keys {
                    let row = *index.get(&FloatOrd(*key)).expect("foreign key");
                    copy_row(row);
                }
            }
            ColumnData::String(ref keys) => {
                let index = match index {
                    UniqueIndex::String(ref i) => i,
                    _ => unreachable!(),
                };
                for key in keys {
                    let row = *index.get(key).expect("foreign key");
                    copy_row(row);
                }
            }
            ColumnData::Date(ref keys) => {
                let index = match index {
                    UniqueIndex::Date(ref i) => i,
                    _ => unreachable!(),
                };
                for key in keys {
                    let row = *index.get(key).expect("foreign key");
                    copy_row(row);
                }
            }
        }

        let mut columns = old_columns;
        columns.extend(new_columns.into_iter());

        let name = crit.new_table_name;

        Table {
            name,
            rows,
            columns,
            name_to_idx,
            unique_indexes,
        }
    }

    fn select(&self, crit: SelectCriteria) -> Table {
        let name = crit.new_table_name;
        let rows = self.rows;
        let mut columns = Vec::new();
        let mut name_to_idx = BTreeMap::new();
        let mut unique_indexes = BTreeMap::new();

        panic!();

        Table {
            name,
            rows,
            columns,
            name_to_idx,
            unique_indexes,
        }
    }
}
