use anyhow::{Result, bail};
use crate::ast::{Schema, Relation};
use std::collections::BTreeSet;
use std::collections::BTreeMap;

type TableName = String;

pub fn check(ast: &Schema) -> Result<TypeInfo> {
    let relation_names = check_relation_names(ast)?;
    let relation_deps = check_relation_preds(ast, &relation_names)?;

    panic!()
}

#[derive(Debug)]
pub struct TypeInfo;

fn check_relation_names(ast: &Schema) -> Result<BTreeSet<TableName>> {
    let mut set = BTreeSet::new();
    for rel in &ast.rels {
        let name = match rel {
            Relation::Table(r) => &r.name,
            Relation::Join(r) => &r.name,
            Relation::View(r) => &r.name,
        };
        if set.contains(name) {
            bail!("relation {} appears multiple times", name);
        }
        let name = name.to_string();
        set.insert(name);
    }

    Ok(set)
}

fn check_relation_preds(ast: &Schema, relation_names: &BTreeSet<TableName>)
                        -> Result<BTreeMap<TableName, BTreeSet<TableName>>> {

    let mut map = BTreeMap::new();
    
    for rel in &ast.rels {
        match rel {
            Relation::Table(table) => {
                assert!(!map.contains_key(&table.name));
                map.insert(table.name.clone(), BTreeSet::new());
            }
            Relation::Join(join) => {
                assert!(!map.contains_key(&join.name));
                let mut set = BTreeSet::new();
                set.insert(join.from.clone());
                for with in &join.with {
                }
            }
            Relation::View(join) => {
            }
        }
    }

    Ok(map)
}
